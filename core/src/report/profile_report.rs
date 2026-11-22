// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::figures::tally::Tally;

// One profile's run: its name, its seed, and what it counted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileReport {
    pub name: &'static str,
    pub seed: u64,
    pub tally: Tally,
}
