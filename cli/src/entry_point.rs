// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;

use stingy4rust::budget::budget::Budget;
use stingy4rust::budget::comparison::Comparison;
use stingy4rust::budget::measured_profile::MeasuredProfile;
use stingy4rust::budget::recording::Recording;

use crate::command_line::action::Action;
use crate::command_line::command::Command;
use crate::command_line::output_format::OutputFormat;
use crate::formats::budget_file::BudgetFile;
use crate::formats::check_json::CheckJson;
use crate::formats::table::Table;
use crate::outcome::Outcome;
use crate::ports::command_runner::CommandRunner;
use crate::ports::file_store::FileStore;
use crate::profiles::host_triple::HostTriple;
use crate::profiles::profile_runner::ProfileRunner;
use crate::profiles::target_finder::TargetFinder;
use crate::unable::Unable;

const BUDGET: &str = "stingy4rust-budget.json";
const OVER: u8 = 1;
const UNABLE: u8 = 2;
const WITHIN: u8 = 0;

// One invocation, composed: the profile target found, run and read, then its
// figures reported, checked against the budget or recorded as it. The one
// place that knows the exit codes -- 0 within budget, gains included; 1 over
// it, unbudgeted or missing; 2 unable to measure, never a verdict.
pub struct EntryPoint<'a> {
    runner: &'a dyn CommandRunner,
    files: &'a dyn FileStore,
}

impl<'a> EntryPoint<'a> {
    #[must_use]
    pub fn new(runner: &'a dyn CommandRunner, files: &'a dyn FileStore) -> Self {
        Self { runner, files }
    }

    #[must_use]
    pub fn run(&self, command: &Command) -> Outcome {
        self.outcome(command).unwrap_or_else(|unable| Outcome {
            stdout: String::new(),
            stderr: format!("stingy4rust: {unable}\n"),
            exit_code: UNABLE,
        })
    }

    fn budget(&self, path: &str) -> Result<Budget, Unable> {
        let text = self
            .files
            .read(path)
            .map_err(|failure| Unable::UnreadableBudget {
                path: String::from(path),
                reason: failure.reason,
            })?;
        text.map_or_else(
            || Ok(Budget::default()),
            |text| BudgetFile::read(path, &text),
        )
    }

    fn outcome(&self, command: &Command) -> Result<Outcome, Unable> {
        let target = TargetFinder::new(self.runner).find(command.package.as_deref())?;
        let triple = match &command.target {
            Some(triple) => triple.clone(),
            None => HostTriple::new(self.runner).read()?,
        };
        let measurement = ProfileRunner::new(self.runner, self.files).measure(
            &target,
            command.profile.as_deref(),
            command.target.as_deref(),
        )?;
        if command.action == Action::Report {
            let stdout = match command.format {
                OutputFormat::Table => Table::report(&measurement.profiles),
                OutputFormat::Json => measurement.report,
            };
            return Ok(Self::printed(stdout, WITHIN));
        }
        let measured: Vec<MeasuredProfile> = measurement
            .profiles
            .into_iter()
            .map(|profile| profile.measured)
            .collect();
        let path = command.budget.clone().unwrap_or_else(|| {
            Path::new(&target.workspace_root)
                .join(BUDGET)
                .to_string_lossy()
                .into_owned()
        });
        let budget = self.budget(&path)?;
        let profile = command.profile.as_deref();
        let checked = Comparison::of(&budget, &measured, &triple, profile);
        if command.action == Action::Record {
            let recorded = Recording::of(&budget, &measured, &triple, profile);
            self.files
                .write(&path, &BudgetFile::write(&recorded))
                .map_err(|failure| Unable::UnwritableBudget {
                    path: path.clone(),
                    reason: failure.reason,
                })?;
            return Ok(Self::printed(
                format!("{}recorded {path}\n", Table::check(&checked)),
                WITHIN,
            ));
        }
        let fails = checked
            .profiles
            .iter()
            .flat_map(|profile| &profile.figures)
            .any(|figure| figure.verdict.fails());
        let stdout = match command.format {
            OutputFormat::Table => Table::check(&checked),
            OutputFormat::Json => CheckJson::write(&checked),
        };
        Ok(Self::printed(stdout, if fails { OVER } else { WITHIN }))
    }

    const fn printed(stdout: String, exit_code: u8) -> Outcome {
        Outcome {
            stdout,
            stderr: String::new(),
            exit_code,
        }
    }
}
