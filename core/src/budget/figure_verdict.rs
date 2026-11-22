// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

use crate::budget::verdict::Verdict;

// One figure compared: its budget and what may be measured against it, if it
// has one; what was measured, if it was; and the verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FigureVerdict {
    pub path: String,
    pub budget: Option<u64>,
    pub measured: Option<u64>,
    pub allowed: Option<u64>,
    pub verdict: Verdict,
}
