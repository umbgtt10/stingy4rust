// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::ports::command_output::CommandOutput;
use crate::ports::invocation::Invocation;
use crate::unable::Unable;

// Where the subcommand runs cargo and rustc: the system, or a test's fake.
pub trait CommandRunner {
    fn run(&self, invocation: &Invocation) -> Result<CommandOutput, Unable>;
}
