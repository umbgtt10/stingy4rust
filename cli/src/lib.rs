// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

//! `cargo stingy4rust`: finds a project's profile target, runs it, and reports
//! its figures, checks them against the budget or records them as it.

pub mod command_line;
pub mod entry_point;
pub mod formats;
pub mod outcome;
pub mod ports;
pub mod profiles;
pub mod system;
pub mod unable;
