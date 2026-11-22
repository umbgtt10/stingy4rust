// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::env::var;

use crate::environment::Environment;

// The process's own environment; a variable that is unset, or not Unicode,
// is none.
pub struct ProcessEnvironment;

impl Environment for ProcessEnvironment {
    fn variable(&self, name: &str) -> Option<String> {
        var(name).ok()
    }
}
