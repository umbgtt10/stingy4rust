// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::io::Error;
use std::io::stderr;
use std::process::Command;
use std::process::Stdio;

use crate::ports::command_output::CommandOutput;
use crate::ports::command_runner::CommandRunner;
use crate::ports::invocation::Invocation;
use crate::unable::Unable;

// Runs a program on this machine. One whose output passes through writes it
// all to the subcommand's standard error, as it goes, so a long build shows
// its progress and a report on standard output stays clean; one whose output
// is kept hands it back.
pub struct SystemCommandRunner;

impl CommandRunner for SystemCommandRunner {
    fn run(&self, invocation: &Invocation) -> Result<CommandOutput, Unable> {
        let mut command = Command::new(&invocation.program);
        command.args(&invocation.arguments).envs(
            invocation
                .environment
                .iter()
                .map(|(name, value)| (name, value)),
        );
        if let Some(directory) = &invocation.directory {
            command.current_dir(directory);
        }
        let launch = |error: Error| Unable::Launch {
            program: invocation.program.clone(),
            reason: error.to_string(),
        };
        if invocation.passes_through {
            let status = command
                .stdout(Stdio::from(stderr()))
                .stderr(Stdio::inherit())
                .status()
                .map_err(launch)?;
            return Ok(CommandOutput {
                code: status.code(),
                stdout: String::new(),
                stderr: String::new(),
            });
        }
        let output = command.output().map_err(launch)?;
        Ok(CommandOutput {
            code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}
