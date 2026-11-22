// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::subcommand_run::SubcommandRun;

// A record and the check that follows it.
pub struct TwoRuns {
    pub record: SubcommandRun,
    pub check: SubcommandRun,
}
