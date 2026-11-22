// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde_json::Value;
use serde_json::from_str;

use crate::ports::command_runner::CommandRunner;
use crate::ports::invocation::Invocation;
use crate::profiles::profile_target::ProfileTarget;
use crate::unable::Unable;

const ARGUMENTS: [&str; 4] = ["metadata", "--format-version", "1", "--no-deps"];
const TARGET: &str = "stingy";

// Which package holds the profiles: from `cargo metadata`, the packages with
// a test target named `stingy`. The one named, if one is; else the only one;
// none, or several with none named, is unable.
pub struct TargetFinder<'a> {
    runner: &'a dyn CommandRunner,
}

impl<'a> TargetFinder<'a> {
    #[must_use]
    pub fn new(runner: &'a dyn CommandRunner) -> Self {
        Self { runner }
    }

    pub fn find(&self, package: Option<&str>) -> Result<ProfileTarget, Unable> {
        let metadata = self.metadata()?;
        let profiled = Self::profiled(&metadata);
        let chosen = match package {
            Some(name) if profiled.iter().any(|candidate| candidate == name) => String::from(name),
            Some(name) => return Err(Unable::NotAProfileTarget(String::from(name))),
            None => match profiled.as_slice() {
                [] => return Err(Unable::NoProfileTarget),
                [only] => only.clone(),
                several => return Err(Unable::SeveralProfileTargets(several.to_vec())),
            },
        };
        Ok(ProfileTarget {
            package: chosen,
            workspace_root: Self::text(&metadata, "workspace_root")?,
            target_directory: Self::text(&metadata, "target_directory")?,
        })
    }

    fn metadata(&self) -> Result<Value, Unable> {
        let output = self.runner.run(&Invocation {
            program: String::from("cargo"),
            arguments: ARGUMENTS.map(String::from).to_vec(),
            directory: None,
            environment: Vec::new(),
            passes_through: false,
        })?;
        if output.code != Some(0) {
            return Err(Unable::Failed {
                command: format!("cargo {}", ARGUMENTS.join(" ")),
                code: output.code,
            });
        }
        from_str(&output.stdout).map_err(|error| Unable::UnreadableMetadata(error.to_string()))
    }

    // The packages with a `stingy` test target, in cargo's order.
    fn profiled(metadata: &Value) -> Vec<String> {
        metadata["packages"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|package| {
                package["targets"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .any(|target| {
                        target["name"] == TARGET
                            && target["kind"]
                                .as_array()
                                .is_some_and(|kinds| kinds.iter().any(|kind| kind == "test"))
                    })
            })
            .filter_map(|package| package["name"].as_str().map(String::from))
            .collect()
    }

    fn text(metadata: &Value, field: &str) -> Result<String, Unable> {
        metadata[field]
            .as_str()
            .map(String::from)
            .ok_or_else(|| Unable::UnreadableMetadata(format!("it names no {field}")))
    }
}
