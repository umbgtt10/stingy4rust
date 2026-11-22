// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::figures::scope_name::ScopeName;

// Where a run's scopes, operations, wire and stores are counted. The ledger
// is one; a test's fake that keeps the calls is another. `Sync`, so a
// `static` can hold one and every handle to it can be `'static`.
pub trait Recorder: Sync {
    fn enter(&self, scope: ScopeName) -> usize;
    fn leave(&self, depth: usize);
    fn begin_operation(&self, name: &'static str);
    fn end_operation(&self, name: &'static str);
    fn sent(&self, kind: &'static str, bytes: u64);
    fn written(&self, store: ScopeName, bytes: u64);
    fn transaction(&self, store: ScopeName);
    fn at_rest(&self, store: ScopeName, bytes: u64);
    fn on_disk(&self, store: ScopeName, bytes: u64);
}
