// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;
use stingy4rust_host::environment::Environment;

// An environment holding only the variables it was given.
pub struct FakeEnvironment {
    variables: BTreeMap<String, String>,
}

impl FakeEnvironment {
    pub const fn new() -> Self {
        Self {
            variables: BTreeMap::new(),
        }
    }

    pub fn with(mut self, name: &str, value: &str) -> Self {
        self.variables
            .insert(String::from(name), String::from(value));
        self
    }
}

impl Environment for FakeEnvironment {
    fn variable(&self, name: &str) -> Option<String> {
        self.variables.get(name).cloned()
    }
}
