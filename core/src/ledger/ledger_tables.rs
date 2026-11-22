// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::figures::heap_figures::HeapFigures;
use crate::figures::scope_name::ScopeName;
use crate::figures::tally::Tally;
use crate::ledger::block_entry::BlockEntry;
use crate::ledger::block_table::BlockTable;
use crate::ledger::bookkeeping::Bookkeeping;
use crate::ledger::frame::Frame;
use crate::ledger::frame_stack::FrameStack;
use crate::ledger::name_event::NameEvent;
use crate::ledger::name_role::NameRole;
use crate::ledger::name_table::NameTable;
use crate::ledger::run_status::RunStatus;
use crate::ledger::scope_table::ScopeTable;

// The state behind the ledger's lock, and the counting rules: each block
// charged to the scope open when it was handed out and credited back to that
// scope whoever frees it; a reallocation frees the old block from its scope and
// charges the new one to the scope open now; another thread's call, a table
// that fills, a frame too deep, a name with a dot or a guard left out of order
// is the run's status -- the first one met -- and is not counted. Nothing here
// allocates or panics but `tally`, which runs once counting has stopped.
pub struct LedgerTables<const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> {
    blocks: BlockTable<BLOCKS>,
    scopes: ScopeTable<SCOPES>,
    names: NameTable<NAMES>,
    frames: FrameStack,
    operation: Option<usize>,
    status: RunStatus,
    thread: usize,
    live: u64,
    peak: u64,
    untracked_frees: u64,
}

impl<const SCOPES: usize, const BLOCKS: usize, const NAMES: usize>
    LedgerTables<SCOPES, BLOCKS, NAMES>
{
    #[must_use]
    pub const fn new() -> Self {
        Self {
            blocks: BlockTable::new(),
            scopes: ScopeTable::new(),
            names: NameTable::new(),
            frames: FrameStack::new(),
            operation: None,
            status: RunStatus::Measured,
            thread: 0,
            live: 0,
            peak: 0,
            untracked_frees: 0,
        }
    }

    pub fn allocated(&mut self, address: usize, size: usize, thread: usize) {
        if self.refused(thread) {
            return;
        }
        let size = size as u64;
        if let Some(slot) = self.tracked(address, size) {
            let live = self.scopes.allocated(slot, size);
            self.charged(slot, live, size);
        }
    }

    pub fn at_rest(&mut self, store: ScopeName, bytes: u64) {
        self.stored(store, NameEvent::AtRest(bytes));
    }

    pub fn begin(&mut self, thread: usize) {
        self.blocks.clear();
        self.scopes.clear();
        self.names.clear();
        self.frames.clear();
        self.operation = None;
        self.status = RunStatus::Measured;
        self.thread = thread;
        self.live = 0;
        self.peak = 0;
        self.untracked_frees = 0;
    }

    // Opens an operation: the wire is charged to it, as well as to each kind,
    // until it ends -- or until another opens, which takes the charge over, as
    // operations do not nest.
    pub fn begin_operation(&mut self, name: &'static str) {
        self.operation = self.named(NameRole::Operation, Self::plain(name));
    }

    #[must_use]
    pub const fn bookkeeping(&self, ledger_bytes: u64) -> Bookkeeping {
        Bookkeeping {
            block_capacity: BLOCKS as u64,
            tracked_blocks_peak: self.blocks.peak() as u64,
            scope_capacity: SCOPES as u64,
            name_capacity: NAMES as u64,
            untracked_frees: self.untracked_frees,
            ledger_bytes,
        }
    }

    // Ends an operation and counts it.
    pub fn end_operation(&mut self, name: &'static str) {
        let slot = self.named(NameRole::Operation, Self::plain(name));
        if let Some(slot) = slot {
            self.names.record(slot, NameEvent::Completed);
        }
        if self.operation == slot {
            self.operation = None;
        }
    }

    // Opens an entry of the scope and hands back its depth, from 1; 0 when the
    // scope could not be opened, which `leave` ignores.
    pub fn enter(&mut self, scope: ScopeName) -> usize {
        if scope.name.contains('.') {
            self.fail(RunStatus::NameWithDot);
            return 0;
        }
        let Some(slot) = self.scopes.intern(scope) else {
            self.fail(RunStatus::ScopesExhausted);
            return 0;
        };
        let base = self.scopes.live(slot);
        let Some(depth) = self.frames.push(Frame {
            slot,
            base,
            high: base,
        }) else {
            self.fail(RunStatus::DepthExhausted);
            return 0;
        };
        self.scopes.entered(slot);
        depth
    }

    pub fn freed(&mut self, address: usize, size: usize, thread: usize) {
        if self.refused(thread) {
            return;
        }
        self.credited(address, size as u64);
    }

    // Closes the entry at `depth`, and any opened inside it and left open,
    // keeping each one's rise; a depth other than the innermost is the run's
    // `ScopeLeftOutOfOrder`.
    pub fn leave(&mut self, depth: usize) {
        if depth == 0 {
            return;
        }
        if depth != self.frames.depth() {
            self.fail(RunStatus::ScopeLeftOutOfOrder);
        }
        while self.frames.depth() >= depth {
            let Some(frame) = self.frames.pop() else {
                return;
            };
            self.scopes
                .rose(frame.slot, frame.high.saturating_sub(frame.base));
        }
    }

    pub fn on_disk(&mut self, store: ScopeName, bytes: u64) {
        self.stored(store, NameEvent::OnDisk(bytes));
    }

    pub fn reallocated(
        &mut self,
        old_address: usize,
        old_size: usize,
        new_address: usize,
        new_size: usize,
        thread: usize,
    ) {
        if self.refused(thread) {
            return;
        }
        let was_tracked = self.credited(old_address, old_size as u64);
        let size = new_size as u64;
        if let Some(slot) = self.tracked(new_address, size) {
            let live = if was_tracked {
                self.scopes.reallocated(slot, size)
            } else {
                self.scopes.allocated(slot, size)
            };
            self.charged(slot, live, size);
        }
    }

    // One message of a kind, charged to the operation open too, if one is.
    pub fn sent(&mut self, kind: &'static str, bytes: u64) {
        if let Some(slot) = self.named(NameRole::Kind, Self::plain(kind)) {
            self.names.record(slot, NameEvent::Counted(bytes));
            if let Some(operation) = self.operation {
                self.names.record(operation, NameEvent::Counted(bytes));
            }
        }
    }

    // What the run counted, its heap the sum of its scopes'. Allocates: call it
    // only once counting has stopped.
    #[must_use]
    pub fn tally(&self, ledger_bytes: u64) -> Tally {
        let scopes = self.scopes.figures();
        let heap = scopes.iter().fold(HeapFigures::default(), |sum, figures| {
            Self::summed(sum, figures.heap)
        });
        Tally {
            status: self.status,
            heap,
            peak: self.peak,
            scopes,
            wire: self.names.wire(),
            storage: self.names.storage(),
            operations: self.names.operations(),
            bookkeeping: self.bookkeeping(ledger_bytes),
        }
    }

    pub fn transaction(&mut self, store: ScopeName) {
        self.stored(store, NameEvent::Completed);
    }

    pub fn written(&mut self, store: ScopeName, bytes: u64) {
        self.stored(store, NameEvent::Counted(bytes));
    }

    // After `size` was charged to `slot`, which now holds `slot_live`: every
    // open entry of the slot and the run's peak see it.
    fn charged(&mut self, slot: usize, slot_live: u64, size: u64) {
        self.frames.raise(slot, slot_live);
        self.live = self.live.saturating_add(size);
        self.peak = self.peak.max(self.live);
    }

    // Credits a block back to the scope it was charged to; a block the table
    // never held is an untracked free. Whether the table held it.
    fn credited(&mut self, address: usize, size: u64) -> bool {
        let Some(entry) = self.blocks.remove(address) else {
            self.untracked_frees = self.untracked_frees.saturating_add(1);
            return false;
        };
        self.scopes.freed(entry.scope, size);
        self.live = self.live.saturating_sub(size);
        true
    }

    const fn fail(&mut self, status: RunStatus) {
        if matches!(self.status, RunStatus::Measured) {
            self.status = status;
        }
    }

    // The slot of a kind, store or operation; a name with a dot is the run's
    // `NameWithDot` and a full table its `NamesExhausted`, and neither is
    // counted.
    fn named(&mut self, role: NameRole, name: ScopeName) -> Option<usize> {
        if name.name.contains('.') {
            self.fail(RunStatus::NameWithDot);
            return None;
        }
        let slot = self.names.intern(role, name);
        if slot.is_none() {
            self.fail(RunStatus::NamesExhausted);
        }
        slot
    }

    const fn plain(name: &'static str) -> ScopeName {
        ScopeName { name, number: None }
    }

    // A call from a thread other than the run's is refused, and is the run's
    // `ForeignThread`.
    const fn refused(&mut self, thread: usize) -> bool {
        if thread == self.thread {
            return false;
        }
        self.fail(RunStatus::ForeignThread);
        true
    }

    // Something happened to a store: counted against it, unless its name
    // cannot be.
    fn stored(&mut self, store: ScopeName, event: NameEvent) {
        if let Some(slot) = self.named(NameRole::Store, store) {
            self.names.record(slot, event);
        }
    }

    const fn summed(sum: HeapFigures, heap: HeapFigures) -> HeapFigures {
        HeapFigures {
            allocations: sum.allocations.saturating_add(heap.allocations),
            reallocations: sum.reallocations.saturating_add(heap.reallocations),
            bytes_allocated: sum.bytes_allocated.saturating_add(heap.bytes_allocated),
            bytes_freed: sum.bytes_freed.saturating_add(heap.bytes_freed),
            live_at_end: sum.live_at_end.saturating_add(heap.live_at_end),
        }
    }

    // Records the block against the scope open now and hands back that scope's
    // slot; a full table is the run's `BlocksExhausted`, and the block goes
    // uncounted.
    fn tracked(&mut self, address: usize, size: u64) -> Option<usize> {
        let slot = self.frames.current().unwrap_or(0);
        if self.blocks.insert(BlockEntry {
            address,
            size,
            scope: slot,
        }) {
            Some(slot)
        } else {
            self.fail(RunStatus::BlocksExhausted);
            None
        }
    }
}

impl<const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> Default
    for LedgerTables<SCOPES, BLOCKS, NAMES>
{
    fn default() -> Self {
        Self::new()
    }
}
