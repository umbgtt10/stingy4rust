// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::process::directory_runner::DirectoryRunner;
use serge4rust::process::launch_failure::LaunchFailure;
use std::process::Command;

pub struct SystemDirectoryRunner;

impl SystemDirectoryRunner {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for SystemDirectoryRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectoryRunner for SystemDirectoryRunner {
    fn run_streaming_in(
        &self,
        directory: &str,
        program: &str,
        args: &[String],
    ) -> Result<Option<i32>, LaunchFailure> {
        Command::new(program)
            .args(args)
            .current_dir(directory)
            .status()
            .map(|status| status.code())
            .map_err(|error| LaunchFailure::new(program, &error.to_string()))
    }
}
