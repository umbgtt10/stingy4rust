// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use clap::Parser;

use crate::command_line::action::Action;
use crate::command_line::command::Command;
use crate::command_line::output_format::OutputFormat;

/// The command line, and what it asks for.
#[derive(Parser)]
#[command(
    name = "cargo-stingy4rust",
    version,
    about = "Hold a Rust codebase to budgets of heap, peak heap, storage and wire bytes",
    long_about = None
)]
pub struct Arguments {
    /// Run as `cargo stingy4rust`, cargo passes "stingy4rust" first.
    #[arg(hide = true, default_value = "")]
    _cargo_subcommand: String,

    #[command(subcommand)]
    action: Action,

    /// The package whose `stingy` test target holds the profiles.
    #[arg(long, global = true)]
    package: Option<String>,

    /// One profile to run, by name; every profile when none is named.
    #[arg(long, global = true)]
    profile: Option<String>,

    /// The target triple to build for and budget under; the host's by default.
    #[arg(long, global = true)]
    target: Option<String>,

    /// How to print: a table, or JSON.
    #[arg(long, global = true, value_enum, default_value = "table")]
    format: OutputFormat,

    /// The budget file; stingy4rust-budget.json at the workspace root by default.
    #[arg(long, global = true)]
    budget: Option<String>,
}

impl Arguments {
    #[must_use]
    pub fn command(&self) -> Command {
        Command {
            action: self.action,
            package: self.package.clone(),
            profile: self.profile.clone(),
            target: self.target.clone(),
            format: self.format,
            budget: self.budget.clone(),
        }
    }
}
