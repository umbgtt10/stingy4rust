// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::budget_copy::BudgetCopy;
use crate::subcommand_run::SubcommandRun;
use crate::two_runs::TwoRuns;

// `record` of the measured fixture into a budget not there yet, then `check`
// against what it wrote.
pub struct RecordScenario;

impl RecordScenario {
    #[must_use]
    pub fn record_then_check() -> TwoRuns {
        let budget = BudgetCopy::absent();
        let path = budget.path();
        let arguments = ["--package", "measured", "--budget", &path];
        TwoRuns {
            record: SubcommandRun::of(&[&["record"], arguments.as_slice()].concat()),
            check: SubcommandRun::of(&[&["check"], arguments.as_slice()].concat()),
        }
    }
}
