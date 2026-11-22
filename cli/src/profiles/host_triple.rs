// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::ports::command_runner::CommandRunner;
use crate::ports::invocation::Invocation;
use crate::unable::Unable;

const HOST: &str = "host: ";

// The target triple a budget is kept under when none is named: the host's,
// from the `host:` line of `rustc -vV`.
pub struct HostTriple<'a> {
    runner: &'a dyn CommandRunner,
}

impl<'a> HostTriple<'a> {
    #[must_use]
    pub fn new(runner: &'a dyn CommandRunner) -> Self {
        Self { runner }
    }

    pub fn read(&self) -> Result<String, Unable> {
        let output = self.runner.run(&Invocation {
            program: String::from("rustc"),
            arguments: vec![String::from("-vV")],
            directory: None,
            environment: Vec::new(),
            passes_through: false,
        })?;
        if output.code != Some(0) {
            return Err(Unable::Failed {
                command: String::from("rustc -vV"),
                code: output.code,
            });
        }
        output
            .stdout
            .lines()
            .find_map(|line| line.strip_prefix(HOST))
            .map(|triple| String::from(triple.trim()))
            .ok_or(Unable::NoHostTriple)
    }
}
