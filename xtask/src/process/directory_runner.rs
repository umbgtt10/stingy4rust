// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serge4rust::process::launch_failure::LaunchFailure;

// A program started in a directory of its own, its output streamed: what the
// own-budget gate needs to run the subcommand in the fixture's workspace as a
// user would, and what serge4rust's CommandRunner, whose programs all start in
// the caller's directory, does not offer.
pub trait DirectoryRunner {
    fn run_streaming_in(
        &self,
        directory: &str,
        program: &str,
        args: &[String],
    ) -> Result<Option<i32>, LaunchFailure>;
}
