// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::process::ExitCode;

use stingy4rust::ledger::run_status::RunStatus;
use stingy4rust::report::profile_report::ProfileReport;
use stingy4rust::report::report::Report;
use stingy4rust::report::report_json::ReportJson;
use stingy4rust::run::measuring::Measuring;
use stingy4rust::run::profile::Profile;
use stingy4rust::run::run::Run;

use crate::environment::Environment;
use crate::report_sink::ReportSink;

const PROFILE: &str = "STINGY4RUST_PROFILE";
const REPORT: &str = "STINGY4RUST_REPORT";
const UNABLE: u8 = 2;

// A profile target's `main`: reads where to write the report and, if one is
// named, which profile to run; runs each profile on this thread -- begin, its
// body, end -- and writes every one to the report, a failed one included.
// Exits 0 only if every run was measured and the report was written; 2, with
// the reason on standard error, if not.
pub struct ProfileMain {
    measuring: &'static dyn Measuring,
    profiles: &'static [Profile],
    environment: &'static dyn Environment,
    sink: &'static dyn ReportSink,
}

impl ProfileMain {
    #[must_use]
    pub fn new(
        measuring: &'static dyn Measuring,
        profiles: &'static [Profile],
        environment: &'static dyn Environment,
        sink: &'static dyn ReportSink,
    ) -> Self {
        Self {
            measuring,
            profiles,
            environment,
            sink,
        }
    }

    pub fn run(&self) -> ExitCode {
        match self.reported() {
            Ok(true) => ExitCode::SUCCESS,
            Ok(false) => ExitCode::from(UNABLE),
            Err(reason) => {
                eprintln!("stingy4rust: {reason}");
                ExitCode::from(UNABLE)
            }
        }
    }

    fn measured(&self, profile: &Profile) -> ProfileReport {
        self.measuring.begin();
        (profile.body)(&Run::new(profile.name, profile.seed, self.measuring));
        ProfileReport {
            name: profile.name,
            seed: profile.seed,
            tally: self.measuring.end(),
        }
    }

    // Whether every run was measured, once the report is written.
    fn reported(&self) -> Result<bool, String> {
        let path = self.environment.variable(REPORT).ok_or_else(|| {
            format!("{REPORT} is not set: it names the file the report is written to")
        })?;
        let profiles = self.selected()?;
        let reports: Vec<ProfileReport> = profiles
            .iter()
            .map(|profile| self.measured(profile))
            .collect();
        let measured = reports
            .iter()
            .all(|report| report.tally.status == RunStatus::Measured);
        self.sink
            .write(&path, &ReportJson::write(&Report { profiles: reports }))
            .map_err(|failure| failure.to_string())?;
        Ok(measured)
    }

    fn selected(&self) -> Result<Vec<&'static Profile>, String> {
        let Some(name) = self.environment.variable(PROFILE) else {
            return Ok(self.profiles.iter().collect());
        };
        let chosen: Vec<&'static Profile> = self
            .profiles
            .iter()
            .filter(|profile| profile.name == name)
            .collect();
        if chosen.is_empty() {
            Err(format!(
                "{PROFILE} names {name}, which no profile is called"
            ))
        } else {
            Ok(chosen)
        }
    }
}
