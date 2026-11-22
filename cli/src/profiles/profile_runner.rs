// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;

use crate::formats::report_file::ReportFile;
use crate::ports::command_runner::CommandRunner;
use crate::ports::file_store::FileStore;
use crate::ports::invocation::Invocation;
use crate::ports::store_failure::StoreFailure;
use crate::profiles::measurement::Measurement;
use crate::profiles::profile_target::ProfileTarget;
use crate::unable::Unable;

const MEASURED: &str = "measured";
const PROFILE: &str = "STINGY4RUST_PROFILE";
const REPORT: &str = "STINGY4RUST_REPORT";

// Runs a project's profile target as the design has it -- `cargo test
// --release --package P --test stingy` in the workspace, its output passing
// through, STINGY4RUST_REPORT naming target/stingy4rust/P.json -- and reads
// the report it wrote. A report left by an earlier run is removed first, so it
// can never be read as this one's. A run not measured, a build that failed or
// a report that cannot be read is unable.
pub struct ProfileRunner<'a> {
    runner: &'a dyn CommandRunner,
    files: &'a dyn FileStore,
}

impl<'a> ProfileRunner<'a> {
    #[must_use]
    pub fn new(runner: &'a dyn CommandRunner, files: &'a dyn FileStore) -> Self {
        Self { runner, files }
    }

    pub fn measure(
        &self,
        target: &ProfileTarget,
        profile: Option<&str>,
        triple: Option<&str>,
    ) -> Result<Measurement, Unable> {
        let report = Path::new(&target.target_directory)
            .join("stingy4rust")
            .join(format!("{}.json", target.package))
            .to_string_lossy()
            .into_owned();
        self.files.remove(&report).map_err(Self::unreadable)?;
        let invocation = Self::invocation(target, &report, profile, triple);
        let code = self.runner.run(&invocation)?.code;
        let failed = Unable::Failed {
            command: format!("cargo {}", invocation.arguments.join(" ")),
            code,
        };
        let Some(text) = self.files.read(&report).map_err(Self::unreadable)? else {
            return Err(failed);
        };
        let profiles = ReportFile::read(&text)?;
        if let Some(unmeasured) = profiles.iter().find(|profile| profile.status != MEASURED) {
            return Err(Unable::NotMeasured {
                profile: unmeasured.measured.name.clone(),
                status: unmeasured.status.clone(),
            });
        }
        if code != Some(0) {
            return Err(failed);
        }
        Ok(Measurement {
            report: text,
            profiles,
        })
    }

    fn invocation(
        target: &ProfileTarget,
        report: &str,
        profile: Option<&str>,
        triple: Option<&str>,
    ) -> Invocation {
        let mut arguments: Vec<String> = [
            "test",
            "--release",
            "--package",
            &target.package,
            "--test",
            "stingy",
        ]
        .map(String::from)
        .to_vec();
        if let Some(triple) = triple {
            arguments.extend([String::from("--target"), String::from(triple)]);
        }
        let mut environment = vec![(String::from(REPORT), String::from(report))];
        if let Some(profile) = profile {
            environment.push((String::from(PROFILE), String::from(profile)));
        }
        Invocation {
            program: String::from("cargo"),
            arguments,
            directory: Some(target.workspace_root.clone()),
            environment,
            passes_through: true,
        }
    }
    fn unreadable(failure: StoreFailure) -> Unable {
        let StoreFailure { path, reason } = failure;
        Unable::UnreadableReport(format!("{path}: {reason}"))
    }
}
