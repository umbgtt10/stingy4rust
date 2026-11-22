// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::sync::OnceLock;

use crate::fixture_run::FixtureRun;
use crate::two_reports::TwoReports;

const MEASURED: &str = "measured";

// The measured fixture's profiles, run once and shared by every scenario
// that reads their figures; and run twice, to compare the two reports.
pub struct FixtureRunScenario;

impl FixtureRunScenario {
    #[must_use]
    pub fn run() -> &'static FixtureRun {
        static RUN: OnceLock<FixtureRun> = OnceLock::new();
        RUN.get_or_init(|| FixtureRun::of(MEASURED))
    }

    #[must_use]
    pub fn run_twice() -> TwoReports {
        let first = FixtureRun::of(MEASURED);
        let second = FixtureRun::of(MEASURED);
        TwoReports {
            first: String::from(first.report()),
            second: String::from(second.report()),
        }
    }
}
