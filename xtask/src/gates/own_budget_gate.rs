// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::process::directory_runner::DirectoryRunner;
use serge4rust::gates::gate::Gate;
use serge4rust::gates::gate_failure::GateFailure;

const BUDGET: &str = "stingy4rust-budget.json";
const OVER: i32 = 1;
const UNABLE: i32 = 2;

// The tool gates itself: this checkout's subcommand, built from source in
// release, checks the fixture against the fixture's own budget, run in the
// fixture's workspace as a user would run it. A tool that enforces a budget
// it does not keep is not worth installing. It fails in its own words, as it
// always has: serge4rust's CouldNotRun would say the code as `Some(2)`.
pub struct OwnBudgetGate<'a> {
    runner: &'a dyn DirectoryRunner,
    manifest_path: String,
    fixture: String,
    package: String,
}

impl<'a> OwnBudgetGate<'a> {
    pub const fn new(
        runner: &'a dyn DirectoryRunner,
        manifest_path: String,
        fixture: String,
        package: String,
    ) -> Self {
        Self {
            runner,
            manifest_path,
            fixture,
            package,
        }
    }
}

impl Gate for OwnBudgetGate<'_> {
    fn label(&self) -> String {
        String::from("Own budget")
    }

    fn run(&self) -> Result<(), GateFailure> {
        let args = [
            "run",
            "--quiet",
            "--release",
            "--manifest-path",
            &self.manifest_path,
            "--bin",
            "cargo-stingy4rust",
            "--",
            "stingy4rust",
            "check",
            "--package",
            &self.package,
        ]
        .map(String::from);

        // 1 is a figure over its budget, unbudgeted or missing; 2 is a run
        // that could not be measured -- never a verdict, so never read as one.
        match self
            .runner
            .run_streaming_in(&self.fixture, "cargo", &args)?
        {
            Some(0) => Ok(()),
            Some(OVER) => Err(GateFailure::Own {
                reason: format!("the fixture's figures moved from {}/{BUDGET}", self.fixture),
            }),
            Some(UNABLE) => Err(GateFailure::Own {
                reason: format!("could not run, exit code {UNABLE}"),
            }),
            code => Err(GateFailure::Own {
                reason: format!("exit code {code:?}"),
            }),
        }
    }
}
