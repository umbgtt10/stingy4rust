// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

use spin::Mutex;

use crate::figures::scope_name::ScopeName;
use crate::figures::tally::Tally;
use crate::ledger::bookkeeping::Bookkeeping;
use crate::ledger::ledger_tables::LedgerTables;
use crate::run::recorder::Recorder;

// Every count of a run, in fixed tables behind a lock that needs no `std` and
// no allocation, so a `static` allocator can hold it. It counts from `begin` to
// `end` and nowhere else: outside a run every call returns at once, so a
// process allocates freely while it starts and while it writes its report.
// `end` stops the counting before it builds the tally, so the tally's own
// allocations reach a ledger that is no longer counting -- and never wait on
// the lock `end` holds.
pub struct Ledger<const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> {
    counting: AtomicBool,
    tables: Mutex<LedgerTables<SCOPES, BLOCKS, NAMES>>,
}

impl<const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> Ledger<SCOPES, BLOCKS, NAMES> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            counting: AtomicBool::new(false),
            tables: Mutex::new(LedgerTables::new()),
        }
    }

    pub fn allocated(&self, address: usize, size: usize, thread: usize) {
        self.while_counting(|tables| tables.allocated(address, size, thread));
    }

    pub fn begin(&self, thread: usize) {
        self.tables.lock().begin(thread);
        self.counting.store(true, Ordering::Release);
    }

    #[must_use]
    pub fn bookkeeping(&self) -> Bookkeeping {
        self.tables.lock().bookkeeping(Self::ledger_bytes())
    }

    pub fn end(&self) -> Tally {
        self.counting.store(false, Ordering::Release);
        self.tables.lock().tally(Self::ledger_bytes())
    }

    pub fn freed(&self, address: usize, size: usize, thread: usize) {
        self.while_counting(|tables| tables.freed(address, size, thread));
    }

    pub fn reallocated(
        &self,
        old_address: usize,
        old_size: usize,
        new_address: usize,
        new_size: usize,
        thread: usize,
    ) {
        self.while_counting(|tables| {
            tables.reallocated(old_address, old_size, new_address, new_size, thread);
        });
    }

    const fn ledger_bytes() -> u64 {
        size_of::<Self>() as u64
    }

    // Hands the tables to `count` during a run; outside one, returns at once
    // with nothing -- no depth, no figure -- and never touches the lock.
    fn while_counting<R: Default>(
        &self,
        count: impl FnOnce(&mut LedgerTables<SCOPES, BLOCKS, NAMES>) -> R,
    ) -> R {
        if self.counting.load(Ordering::Acquire) {
            count(&mut self.tables.lock())
        } else {
            R::default()
        }
    }
}

impl<const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> Default
    for Ledger<SCOPES, BLOCKS, NAMES>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> Recorder
    for Ledger<SCOPES, BLOCKS, NAMES>
{
    fn at_rest(&self, store: ScopeName, bytes: u64) {
        self.while_counting(|tables| tables.at_rest(store, bytes));
    }

    fn begin_operation(&self, name: &'static str) {
        self.while_counting(|tables| tables.begin_operation(name));
    }

    fn end_operation(&self, name: &'static str) {
        self.while_counting(|tables| tables.end_operation(name));
    }

    fn enter(&self, scope: ScopeName) -> usize {
        self.while_counting(|tables| tables.enter(scope))
    }

    fn leave(&self, depth: usize) {
        self.while_counting(|tables| tables.leave(depth));
    }

    fn on_disk(&self, store: ScopeName, bytes: u64) {
        self.while_counting(|tables| tables.on_disk(store, bytes));
    }

    fn sent(&self, kind: &'static str, bytes: u64) {
        self.while_counting(|tables| tables.sent(kind, bytes));
    }

    fn transaction(&self, store: ScopeName) {
        self.while_counting(|tables| tables.transaction(store));
    }

    fn written(&self, store: ScopeName, bytes: u64) {
        self.while_counting(|tables| tables.written(store, bytes));
    }
}
