// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use spin::Mutex;
use stingy4rust::figures::scope_name::ScopeName;
use stingy4rust::run::recorder::Recorder;

// A recorder that keeps every call it is handed, as a line of text, and
// answers each `enter` with the depth it was built with.
pub struct RecordingRecorder {
    calls: Mutex<Vec<String>>,
    depth: usize,
}

impl RecordingRecorder {
    pub const fn new(depth: usize) -> Self {
        Self {
            calls: Mutex::new(Vec::new()),
            depth,
        }
    }

    pub fn calls(&self) -> Vec<String> {
        self.calls.lock().clone()
    }

    fn keep(&self, call: String) {
        self.calls.lock().push(call);
    }

    fn named(scope: ScopeName) -> String {
        scope.number.map_or_else(
            || String::from(scope.name),
            |number| format!("{} {number}", scope.name),
        )
    }
}

impl Recorder for RecordingRecorder {
    fn at_rest(&self, store: ScopeName, bytes: u64) {
        self.keep(format!("at rest {} {bytes}", Self::named(store)));
    }

    fn begin_operation(&self, name: &'static str) {
        self.keep(format!("begin operation {name}"));
    }

    fn end_operation(&self, name: &'static str) {
        self.keep(format!("end operation {name}"));
    }

    fn enter(&self, scope: ScopeName) -> usize {
        self.keep(format!("enter {}", Self::named(scope)));
        self.depth
    }

    fn leave(&self, depth: usize) {
        self.keep(format!("leave {depth}"));
    }

    fn on_disk(&self, store: ScopeName, bytes: u64) {
        self.keep(format!("on disk {} {bytes}", Self::named(store)));
    }

    fn sent(&self, kind: &'static str, bytes: u64) {
        self.keep(format!("sent {kind} {bytes}"));
    }

    fn transaction(&self, store: ScopeName) {
        self.keep(format!("transaction {}", Self::named(store)));
    }

    fn written(&self, store: ScopeName, bytes: u64) {
        self.keep(format!("written {} {bytes}", Self::named(store)));
    }
}
