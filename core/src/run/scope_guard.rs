// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::run::recorder::Recorder;

// An open scope: dropping it leaves the scope at the depth it was entered at,
// so a guard dropped out of order is seen as such.
pub struct ScopeGuard {
    recorder: &'static dyn Recorder,
    depth: usize,
}

impl ScopeGuard {
    #[must_use]
    pub const fn new(recorder: &'static dyn Recorder, depth: usize) -> Self {
        Self { recorder, depth }
    }
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        self.recorder.leave(self.depth);
    }
}
