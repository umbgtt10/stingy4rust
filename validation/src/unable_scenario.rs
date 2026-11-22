// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::budget_copy::BudgetCopy;
use crate::subcommand_run::SubcommandRun;

// `check` where nothing can be measured: a fixture package whose run fails,
// or has no profile target, or a budget that cannot be read.
pub struct UnableScenario;

impl UnableScenario {
    #[must_use]
    pub fn check_of(package: &str) -> SubcommandRun {
        let budget = BudgetCopy::of_fixture();
        SubcommandRun::of(&["check", "--package", package, "--budget", &budget.path()])
    }

    #[must_use]
    pub fn check_with_a_corrupt_budget() -> SubcommandRun {
        let budget = BudgetCopy::of_fixture().corrupted();
        SubcommandRun::of(&["check", "--package", "measured", "--budget", &budget.path()])
    }
}
