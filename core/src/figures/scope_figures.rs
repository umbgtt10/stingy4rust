// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::figures::heap_figures::HeapFigures;
use crate::figures::scope_name::ScopeName;

// One scope's figures: its heap, the most bytes ever live in it, the largest
// rise inside any one entry of it, and how many times it was entered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScopeFigures {
    pub name: ScopeName,
    pub heap: HeapFigures,
    pub peak: u64,
    pub largest_entry_rise: u64,
    pub entries: u64,
}
