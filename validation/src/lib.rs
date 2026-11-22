// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

//! End-to-end scenarios of the stingy4rust crates against the fixture
//! projects: each runs a fixture's profile target, or the built subcommand
//! on the fixture, as a user would, and reads what it wrote.

pub mod budget_copy;
pub mod check_scenario;
pub mod fixture_run;
pub mod fixture_run_scenario;
pub mod record_scenario;
pub mod report_scenario;
pub mod subcommand_run;
pub mod two_reports;
pub mod two_runs;
pub mod unable_scenario;
