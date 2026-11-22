// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// What the counting itself cost and held: its tables' capacities, the most
// blocks it tracked at once, the frees of blocks it never saw allocated, and
// the ledger's own size -- reported, never budgeted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bookkeeping {
    pub block_capacity: u64,
    pub tracked_blocks_peak: u64,
    pub scope_capacity: u64,
    pub name_capacity: u64,
    pub untracked_frees: u64,
    pub ledger_bytes: u64,
}
