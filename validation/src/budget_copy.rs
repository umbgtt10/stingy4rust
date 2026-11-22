// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs::copy;
use std::fs::read_to_string;
use std::fs::write;
use std::path::Path;
use std::path::PathBuf;

use serde_json::Value;
use serde_json::from_str;
use serde_json::to_string_pretty;
use tempfile::TempDir;

const BUDGET: &str = "stingy4rust-budget.json";

// A budget file of a scenario's own, in a scratch directory: absent, or the
// fixture's copied and then edited -- a figure set or removed under every
// target, or the whole file made unreadable -- so no scenario touches the
// fixture's own.
pub struct BudgetCopy {
    directory: TempDir,
}

impl BudgetCopy {
    #[must_use]
    pub fn absent() -> Self {
        Self {
            directory: TempDir::new().expect("a scratch directory"),
        }
    }

    #[must_use]
    pub fn of_fixture() -> Self {
        let budget = Self::absent();
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("fixture")
            .join(BUDGET);
        copy(fixture, budget.file()).expect("the fixture's budget copies");
        budget
    }

    #[must_use]
    pub fn corrupted(self) -> Self {
        write(self.file(), "not json").expect("the copy is writable");
        self
    }

    #[must_use]
    pub fn path(&self) -> String {
        self.file().to_string_lossy().into_owned()
    }

    #[must_use]
    pub fn with_figure(self, profile: &str, path: &str, value: Option<u64>) -> Self {
        let text = read_to_string(self.file()).expect("the copy");
        let mut budget: Value = from_str(&text).expect("the copy is JSON");
        for profiles in budget["targets"]
            .as_object_mut()
            .expect("the budget has targets")
            .values_mut()
        {
            let figures = profiles[profile]["figures"]
                .as_object_mut()
                .expect("the profile is budgeted");
            match value {
                Some(value) => figures.insert(String::from(path), Value::from(value)),
                None => figures.remove(path),
            };
        }
        write(self.file(), to_string_pretty(&budget).expect("JSON")).expect("the copy is writable");
        self
    }

    fn file(&self) -> PathBuf {
        self.directory.path().join(BUDGET)
    }
}
