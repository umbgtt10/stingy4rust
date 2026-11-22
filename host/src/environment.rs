// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// Where a profile target reads its settings: the process's environment, or a
// test's fake.
pub trait Environment: Sync {
    fn variable(&self, name: &str) -> Option<String>;
}
