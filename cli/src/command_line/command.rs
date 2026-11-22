// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::command_line::action::Action;
use crate::command_line::output_format::OutputFormat;

// What one invocation asks for: the action, and the package, profile, target,
// format and budget file it names -- none of them required.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
    pub action: Action,
    pub package: Option<String>,
    pub profile: Option<String>,
    pub target: Option<String>,
    pub format: OutputFormat,
    pub budget: Option<String>,
}
