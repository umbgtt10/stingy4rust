// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fake_file_store::FakeFileStore;
use std::cell::RefCell;
use stingy4rust_cli::ports::command_output::CommandOutput;
use stingy4rust_cli::ports::command_runner::CommandRunner;
use stingy4rust_cli::ports::invocation::Invocation;
use stingy4rust_cli::unable::Unable;

// Answers each command -- keyed by its program and first argument -- as it was
// told to, keeps every invocation, and when `cargo test` runs writes the
// report it was given into the fake store, as a profile target would.
pub struct FakeCommandRunner<'a> {
    answers: Vec<(String, CommandOutput)>,
    report: Option<(&'a FakeFileStore, String, String)>,
    calls: RefCell<Vec<Invocation>>,
}

impl<'a> FakeCommandRunner<'a> {
    pub const fn new() -> Self {
        Self {
            answers: Vec::new(),
            report: None,
            calls: RefCell::new(Vec::new()),
        }
    }

    pub fn answering(mut self, command: &str, code: Option<i32>, stdout: &str) -> Self {
        self.answers.push((
            String::from(command),
            CommandOutput {
                code,
                stdout: String::from(stdout),
                stderr: String::new(),
            },
        ));
        self
    }

    pub fn calls(&self) -> Vec<Invocation> {
        self.calls.borrow().clone()
    }

    pub fn writing_report(mut self, store: &'a FakeFileStore, path: &str, text: &str) -> Self {
        self.report = Some((store, String::from(path), String::from(text)));
        self
    }
}

impl CommandRunner for FakeCommandRunner<'_> {
    fn run(&self, invocation: &Invocation) -> Result<CommandOutput, Unable> {
        self.calls.borrow_mut().push(invocation.clone());
        let key = format!(
            "{} {}",
            invocation.program,
            invocation.arguments.first().map_or("", String::as_str)
        );
        if key == "cargo test"
            && let Some((store, path, text)) = &self.report
        {
            store.put(path, text);
        }
        self.answers
            .iter()
            .find(|(command, _)| *command == key)
            .map(|(_, output)| output.clone())
            .ok_or_else(|| Unable::Launch {
                program: invocation.program.clone(),
                reason: String::from("the fake has no answer"),
            })
    }
}
