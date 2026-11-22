// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::figures::scope_name::ScopeName;
use crate::run::recorder::Recorder;

// What a project's storage decorator calls for one store: each write and its
// bytes, each transaction, and at the end what the store holds and its files'
// size on disk.
#[derive(Clone, Copy)]
pub struct StorageMeter {
    recorder: &'static dyn Recorder,
    store: ScopeName,
}

impl StorageMeter {
    #[must_use]
    pub const fn new(recorder: &'static dyn Recorder, store: ScopeName) -> Self {
        Self { recorder, store }
    }

    pub fn at_rest(&self, bytes: u64) {
        self.recorder.at_rest(self.store, bytes);
    }

    pub fn on_disk(&self, bytes: u64) {
        self.recorder.on_disk(self.store, bytes);
    }

    pub fn transaction(&self) {
        self.recorder.transaction(self.store);
    }

    pub fn written(&self, bytes: u64) {
        self.recorder.written(self.store, bytes);
    }
}
