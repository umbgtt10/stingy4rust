// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::VecDeque;
use std::sync::Mutex;
use stingy4rust::figures::heap_figures::HeapFigures;
use stingy4rust::figures::scope_name::ScopeName;
use stingy4rust::figures::tally::Tally;
use stingy4rust::ledger::bookkeeping::Bookkeeping;
use stingy4rust::ledger::run_status::RunStatus;
use stingy4rust::run::measuring::Measuring;
use stingy4rust::run::recorder::Recorder;

// A measuring that keeps every begin, end and scope entered as a line, and
// ends each run with the next status it was given -- measured once they run
// out.
pub struct FakeMeasuring {
    calls: Mutex<Vec<String>>,
    statuses: Mutex<VecDeque<RunStatus>>,
}

impl FakeMeasuring {
    pub fn new(statuses: &[RunStatus]) -> Self {
        Self {
            calls: Mutex::new(Vec::new()),
            statuses: Mutex::new(statuses.iter().copied().collect()),
        }
    }

    pub fn calls(&self) -> Vec<String> {
        self.calls
            .lock()
            .expect("the calls are not poisoned")
            .clone()
    }

    fn keep(&self, call: String) {
        self.calls
            .lock()
            .expect("the calls are not poisoned")
            .push(call);
    }
}

impl Recorder for FakeMeasuring {
    fn at_rest(&self, _store: ScopeName, _bytes: u64) {}

    fn begin_operation(&self, _name: &'static str) {}

    fn end_operation(&self, _name: &'static str) {}

    fn enter(&self, scope: ScopeName) -> usize {
        self.keep(format!("enter {}", scope.name));
        1
    }

    fn leave(&self, _depth: usize) {}

    fn on_disk(&self, _store: ScopeName, _bytes: u64) {}

    fn sent(&self, _kind: &'static str, _bytes: u64) {}

    fn transaction(&self, _store: ScopeName) {}

    fn written(&self, _store: ScopeName, _bytes: u64) {}
}

impl Measuring for FakeMeasuring {
    fn begin(&self) {
        self.keep(String::from("begin"));
    }

    fn end(&self) -> Tally {
        self.keep(String::from("end"));
        let status = self
            .statuses
            .lock()
            .expect("the statuses are not poisoned")
            .pop_front()
            .unwrap_or(RunStatus::Measured);
        Tally {
            status,
            heap: HeapFigures::default(),
            peak: 0,
            scopes: Vec::new(),
            wire: Vec::new(),
            storage: Vec::new(),
            operations: Vec::new(),
            bookkeeping: Bookkeeping {
                block_capacity: 8,
                tracked_blocks_peak: 0,
                scope_capacity: 4,
                name_capacity: 4,
                untracked_frees: 0,
                ledger_bytes: 0,
            },
        }
    }
}
