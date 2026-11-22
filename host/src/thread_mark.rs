// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::ptr::from_ref;

thread_local! {
    static MARK: u8 = const { 0 };
}

// What tells the calling thread from every other: the address of a
// thread-local initialised at compile time, so taking it never allocates --
// it is read inside the allocator. Never 0 on a thread that is running; 0
// only on one so far into its teardown that its locals are gone.
pub struct ThreadMark;

impl ThreadMark {
    #[must_use]
    pub fn current() -> usize {
        MARK.try_with(|mark| from_ref(mark).addr()).unwrap_or(0)
    }
}
