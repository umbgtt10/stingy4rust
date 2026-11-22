// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::budget_copy::BudgetCopy;
use crate::subcommand_run::SubcommandRun;

// `check` of the measured fixture against a budget of the scenario's own.
pub struct CheckScenario;

impl CheckScenario {
    #[must_use]
    pub fn check(budget: &BudgetCopy) -> SubcommandRun {
        SubcommandRun::of(&["check", "--package", "measured", "--budget", &budget.path()])
    }
}
