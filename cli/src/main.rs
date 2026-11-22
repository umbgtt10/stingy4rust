// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::args;
use std::io::Write;
use std::io::stderr;
use std::io::stdout;
use std::process::ExitCode;

use clap::Parser;
use stingy4rust_cli::command_line::arguments::Arguments;
use stingy4rust_cli::entry_point::EntryPoint;
use stingy4rust_cli::system::disk_file_store::DiskFileStore;
use stingy4rust_cli::system::system_command_runner::SystemCommandRunner;

// The command line in, the system's adapters wired, the outcome printed and
// its exit code returned -- all a test cannot reach.
fn main() -> ExitCode {
    let arguments = Arguments::parse_from(args());
    let outcome = EntryPoint::new(&SystemCommandRunner, &DiskFileStore).run(&arguments.command());
    let _ = stdout().write_all(outcome.stdout.as_bytes());
    let _ = stderr().write_all(outcome.stderr.as_bytes());
    ExitCode::from(outcome.exit_code)
}
