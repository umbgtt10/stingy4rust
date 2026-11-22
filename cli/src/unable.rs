// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt;

// Why the figures could not be measured, or a budget could not be read or
// written -- every one of them exit code 2, never a verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unable {
    NoProfileTarget,
    SeveralProfileTargets(Vec<String>),
    NotAProfileTarget(String),
    Launch { program: String, reason: String },
    Failed { command: String, code: Option<i32> },
    UnreadableMetadata(String),
    NoHostTriple,
    NotMeasured { profile: String, status: String },
    UnreadableReport(String),
    UnreadableBudget { path: String, reason: String },
    UnwritableBudget { path: String, reason: String },
}

impl fmt::Display for Unable {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoProfileTarget => write!(
                formatter,
                "no package of the workspace has a test target named stingy"
            ),
            Self::SeveralProfileTargets(packages) => write!(
                formatter,
                "several packages have a test target named stingy -- name one with --package: {}",
                packages.join(", ")
            ),
            Self::NotAProfileTarget(package) => {
                write!(
                    formatter,
                    "package {package} has no test target named stingy"
                )
            }
            Self::Launch { program, reason } => {
                write!(formatter, "could not start {program}: {reason}")
            }
            Self::Failed {
                command,
                code: Some(code),
            } => write!(formatter, "{command} failed with exit code {code}"),
            Self::Failed {
                command,
                code: None,
            } => {
                write!(formatter, "{command} was ended by a signal")
            }
            Self::UnreadableMetadata(reason) => {
                write!(formatter, "cargo metadata could not be read: {reason}")
            }
            Self::NoHostTriple => write!(formatter, "rustc -vV named no host"),
            Self::NotMeasured { profile, status } => {
                write!(
                    formatter,
                    "profile {profile} could not be measured: {status}"
                )
            }
            Self::UnreadableReport(reason) => {
                write!(formatter, "the report could not be read: {reason}")
            }
            Self::UnreadableBudget { path, reason } => {
                write!(formatter, "the budget {path} could not be read: {reason}")
            }
            Self::UnwritableBudget { path, reason } => {
                write!(
                    formatter,
                    "the budget {path} could not be written: {reason}"
                )
            }
        }
    }
}
