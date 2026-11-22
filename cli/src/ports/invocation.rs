// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// A program to run: its arguments, the directory to run it in, the variables
// to add to its environment, and whether its output passes through to the
// subcommand's standard error rather than being kept.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub program: String,
    pub arguments: Vec<String>,
    pub directory: Option<String>,
    pub environment: Vec<(String, String)>,
    pub passes_through: bool,
}
