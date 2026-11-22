// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::consts::EXE_SUFFIX;
use std::env::current_exe;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use std::sync::PoisonError;

static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

// One run of the built `cargo-stingy4rust`, as cargo runs it -- `stingy4rust`
// first -- in the fixture workspace, keeping what it printed and its exit
// code. Runs go one at a time: a package's report is one file under the
// fixture's target directory, and two runs at once would each remove the
// other's. The binary is the one built beside this test binary, which a
// workspace test run builds.
pub struct SubcommandRun {
    stdout: String,
    stderr: String,
    exit_code: Option<i32>,
}

impl SubcommandRun {
    #[must_use]
    pub fn of(arguments: &[&str]) -> Self {
        let _one = ONE_AT_A_TIME.lock().unwrap_or_else(PoisonError::into_inner);
        let mut binary = current_exe().expect("the test binary knows where it is");
        binary.pop();
        if binary.ends_with("deps") {
            binary.pop();
        }
        binary.push(format!("cargo-stingy4rust{EXE_SUFFIX}"));
        let output = Command::new(&binary)
            .arg("stingy4rust")
            .args(arguments)
            .current_dir(
                Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("..")
                    .join("fixture"),
            )
            .output()
            .expect("the built subcommand starts");
        Self {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            exit_code: output.status.code(),
        }
    }

    #[must_use]
    pub const fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    #[must_use]
    pub fn stderr(&self) -> &str {
        &self.stderr
    }

    #[must_use]
    pub fn stdout(&self) -> &str {
        &self.stdout
    }
}
