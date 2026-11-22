// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use clap::Subcommand;

// What the subcommand is asked to do with a run: print its figures, hold
// them against the budget, or write them as the budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Subcommand)]
pub enum Action {
    /// Run the profiles and print their figures; no budget is read.
    Report,
    /// Run the profiles and hold their figures against the budget; never writes.
    Check,
    /// Run the profiles and write their figures as this target's budget.
    Record,
}
