// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::figures::scope_name::ScopeName;

// One store: what was written to it, in how many writes and transactions, what
// it holds at the end, and its files' size where it has a disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreFigures {
    pub store: ScopeName,
    pub bytes_written: u64,
    pub writes: u64,
    pub transactions: u64,
    pub bytes_at_rest: u64,
    pub bytes_on_disk: Option<u64>,
}
