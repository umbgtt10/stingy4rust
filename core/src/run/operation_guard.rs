// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::run::recorder::Recorder;

// An open operation: dropping it ends the operation of its name.
pub struct OperationGuard {
    recorder: &'static dyn Recorder,
    name: &'static str,
}

impl OperationGuard {
    #[must_use]
    pub const fn new(recorder: &'static dyn Recorder, name: &'static str) -> Self {
        Self { recorder, name }
    }
}

impl Drop for OperationGuard {
    fn drop(&mut self) {
        self.recorder.end_operation(self.name);
    }
}
