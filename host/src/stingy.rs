// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

#![allow(unsafe_code)]

use std::alloc::GlobalAlloc;
use std::alloc::Layout;

use stingy4rust::figures::scope_name::ScopeName;
use stingy4rust::figures::tally::Tally;
use stingy4rust::ledger::ledger::Ledger;
use stingy4rust::run::measuring::Measuring;
use stingy4rust::run::recorder::Recorder;

use crate::thread_mark::ThreadMark;

// A counting allocator: every call goes to the inner allocator, and what it
// hands out and takes back is counted in the ledger, by the calling thread's
// mark. The only `unsafe` in the workspace, and the only file allowed it. A run is begun and ended through `Measuring`, and its body reaches
// the ledger through `Recorder`.
pub struct Stingy<A, const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> {
    inner: A,
    ledger: Ledger<SCOPES, BLOCKS, NAMES>,
}

impl<A, const SCOPES: usize, const BLOCKS: usize, const NAMES: usize>
    Stingy<A, SCOPES, BLOCKS, NAMES>
{
    #[must_use]
    pub const fn new(inner: A) -> Self {
        Self {
            inner,
            ledger: Ledger::new(),
        }
    }

    // A block the inner allocator handed out is counted; a null, no block,
    // is not.
    fn counted(&self, block: *mut u8, layout: Layout) -> *mut u8 {
        if !block.is_null() {
            self.ledger
                .allocated(block.addr(), layout.size(), ThreadMark::current());
        }
        block
    }
}

// Sound because it changes nothing the inner allocator does: every call is the
// inner allocator's, with the same arguments, and its result is handed back
// as it came. The ledger reads addresses and sizes only, never the memory; it
// never allocates and never panics, so it can neither recurse into this
// allocator nor unwind out of it. A free is counted before the block goes
// back, so its address cannot be handed out again before the ledger forgets
// it.
unsafe impl<A: GlobalAlloc, const SCOPES: usize, const BLOCKS: usize, const NAMES: usize>
    GlobalAlloc for Stingy<A, SCOPES, BLOCKS, NAMES>
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.counted(unsafe { self.inner.alloc(layout) }, layout)
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        self.counted(unsafe { self.inner.alloc_zeroed(layout) }, layout)
    }

    unsafe fn dealloc(&self, block: *mut u8, layout: Layout) {
        self.ledger
            .freed(block.addr(), layout.size(), ThreadMark::current());
        unsafe { self.inner.dealloc(block, layout) }
    }

    unsafe fn realloc(&self, block: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let moved = unsafe { self.inner.realloc(block, layout, new_size) };
        if !moved.is_null() {
            self.ledger.reallocated(
                block.addr(),
                layout.size(),
                moved.addr(),
                new_size,
                ThreadMark::current(),
            );
        }
        moved
    }
}

impl<A: Sync, const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> Recorder
    for Stingy<A, SCOPES, BLOCKS, NAMES>
{
    fn at_rest(&self, store: ScopeName, bytes: u64) {
        self.ledger.at_rest(store, bytes);
    }

    fn begin_operation(&self, name: &'static str) {
        self.ledger.begin_operation(name);
    }

    fn end_operation(&self, name: &'static str) {
        self.ledger.end_operation(name);
    }

    fn enter(&self, scope: ScopeName) -> usize {
        self.ledger.enter(scope)
    }

    fn leave(&self, depth: usize) {
        self.ledger.leave(depth);
    }

    fn on_disk(&self, store: ScopeName, bytes: u64) {
        self.ledger.on_disk(store, bytes);
    }

    fn sent(&self, kind: &'static str, bytes: u64) {
        self.ledger.sent(kind, bytes);
    }

    fn transaction(&self, store: ScopeName) {
        self.ledger.transaction(store);
    }

    fn written(&self, store: ScopeName, bytes: u64) {
        self.ledger.written(store, bytes);
    }
}

impl<A: Sync, const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> Measuring
    for Stingy<A, SCOPES, BLOCKS, NAMES>
{
    fn begin(&self) {
        self.ledger.begin(ThreadMark::current());
    }

    fn end(&self) -> Tally {
        self.ledger.end()
    }
}
