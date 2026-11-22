// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::subcommand_run::SubcommandRun;

// `report` of the measured fixture: no budget read.
pub struct ReportScenario;

impl ReportScenario {
    #[must_use]
    pub fn report() -> SubcommandRun {
        SubcommandRun::of(&["report", "--package", "measured"])
    }
}
