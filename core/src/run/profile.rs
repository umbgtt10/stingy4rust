// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::run::run::Run;

// One line of a project's registry of load profiles: its name, its seed, and
// the function that runs it.
#[derive(Clone, Copy)]
pub struct Profile {
    pub name: &'static str,
    pub seed: u64,
    pub body: fn(&Run),
}
