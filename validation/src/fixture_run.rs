// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::var_os;
use std::ffi::OsString;
use std::fs::read_to_string;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;
use serde_json::from_str;
use tempfile::TempDir;

// One run of a fixture package's profile target, as a user runs it: `cargo
// test --release --test stingy` in the package, with STINGY4RUST_REPORT
// pointing at a scratch file. Keeps the report it wrote, cargo's standard
// error, and the exit code.
pub struct FixtureRun {
    report: String,
    log: String,
    exit_code: Option<i32>,
}

impl FixtureRun {
    #[must_use]
    pub fn of(package: &str) -> Self {
        let scratch = TempDir::new().expect("a scratch directory for the report");
        let report_path = scratch.path().join("report.json");
        let output = Command::new(var_os("CARGO").unwrap_or_else(|| OsString::from("cargo")))
            .args(["test", "--quiet", "--release", "--test", "stingy"])
            .current_dir(Self::fixture(package))
            .env("STINGY4RUST_REPORT", &report_path)
            .env_remove("STINGY4RUST_PROFILE")
            .output()
            .expect("cargo starts");
        Self {
            report: read_to_string(&report_path).unwrap_or_default(),
            log: String::from_utf8_lossy(&output.stderr).into_owned(),
            exit_code: output.status.code(),
        }
    }

    #[must_use]
    pub const fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    // A figure of one profile, by its path; none if the report lacks either.
    #[must_use]
    pub fn figure(&self, profile: &str, path: &str) -> Option<u64> {
        let document: Value = from_str(&self.report).ok()?;
        document["profiles"]
            .as_array()?
            .iter()
            .find(|run| run["name"] == profile)?["figures"]
            .get(path)?
            .as_u64()
    }

    #[must_use]
    pub fn log(&self) -> &str {
        &self.log
    }

    #[must_use]
    pub fn report(&self) -> &str {
        &self.report
    }

    fn fixture(package: &str) -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("fixture")
            .join(package)
    }
}
