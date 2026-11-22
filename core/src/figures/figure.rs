// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

// One figure of a run: its path, its parts joined by `.`, and its value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Figure {
    pub path: String,
    pub value: u64,
}
