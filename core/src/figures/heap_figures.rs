// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// The heap a run, or one scope of it, cost: allocations and reallocations
// counted, bytes handed out and given back, and the bytes still live when it
// ended -- every size the one the code requested.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct HeapFigures {
    pub allocations: u64,
    pub reallocations: u64,
    pub bytes_allocated: u64,
    pub bytes_freed: u64,
    pub live_at_end: u64,
}
