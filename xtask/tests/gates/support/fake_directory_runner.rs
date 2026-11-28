// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serge4rust::process::launch_failure::LaunchFailure;
use std::cell::RefCell;
use xtask::process::directory_runner::DirectoryRunner;

// The seam the own-budget gate is constructed against, faked so its tests see
// where each call ran, what it ran, and with which arguments.
pub struct FakeDirectoryRunner {
    launches: bool,
    streaming_code: Option<i32>,
    calls: RefCell<Vec<Vec<String>>>,
    programs: RefCell<Vec<String>>,
    directories: RefCell<Vec<String>>,
}

impl FakeDirectoryRunner {
    pub const fn new() -> Self {
        Self {
            launches: true,
            streaming_code: Some(0),
            calls: RefCell::new(Vec::new()),
            programs: RefCell::new(Vec::new()),
            directories: RefCell::new(Vec::new()),
        }
    }

    // Every call fails to start the program, as a cargo that is not there
    // would.
    pub const fn with_launch_failure(mut self) -> Self {
        self.launches = false;
        self
    }

    pub const fn with_streaming_code(mut self, streaming_code: Option<i32>) -> Self {
        self.streaming_code = streaming_code;
        self
    }

    pub fn directories(&self) -> Vec<String> {
        self.directories.borrow().clone()
    }

    pub fn calls(&self) -> Vec<Vec<String>> {
        self.calls.borrow().clone()
    }

    // Recorded separately from the arguments, because a caller that started
    // invoking the wrong program with the right arguments would otherwise look
    // identical to one that worked.
    pub fn programs(&self) -> Vec<String> {
        self.programs.borrow().clone()
    }
}

impl Default for FakeDirectoryRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl DirectoryRunner for FakeDirectoryRunner {
    fn run_streaming_in(
        &self,
        directory: &str,
        program: &str,
        args: &[String],
    ) -> Result<Option<i32>, LaunchFailure> {
        self.directories.borrow_mut().push(String::from(directory));
        self.programs.borrow_mut().push(String::from(program));
        self.calls.borrow_mut().push(args.to_vec());
        if self.launches {
            Ok(self.streaming_code)
        } else {
            Err(LaunchFailure::new(program, "refused by the fake"))
        }
    }
}
