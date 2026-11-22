// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::cell::RefCell;
use std::collections::BTreeMap;
use stingy4rust_cli::ports::file_store::FileStore;
use stingy4rust_cli::ports::store_failure::StoreFailure;

// Files held in memory, by path; one that refuses every write if asked to.
pub struct FakeFileStore {
    files: RefCell<BTreeMap<String, String>>,
    refuses_writes: bool,
}

impl FakeFileStore {
    pub const fn new() -> Self {
        Self {
            files: RefCell::new(BTreeMap::new()),
            refuses_writes: false,
        }
    }

    pub const fn refusing_writes(mut self) -> Self {
        self.refuses_writes = true;
        self
    }

    pub fn with(self, path: &str, text: &str) -> Self {
        self.files
            .borrow_mut()
            .insert(String::from(path), String::from(text));
        self
    }

    pub fn file(&self, path: &str) -> Option<String> {
        self.files.borrow().get(path).cloned()
    }

    pub fn put(&self, path: &str, text: &str) {
        self.files
            .borrow_mut()
            .insert(String::from(path), String::from(text));
    }
}

impl FileStore for FakeFileStore {
    fn read(&self, path: &str) -> Result<Option<String>, StoreFailure> {
        Ok(self.file(path))
    }

    fn remove(&self, path: &str) -> Result<(), StoreFailure> {
        self.files.borrow_mut().remove(path);
        Ok(())
    }

    fn write(&self, path: &str, text: &str) -> Result<(), StoreFailure> {
        if self.refuses_writes {
            return Err(StoreFailure {
                path: String::from(path),
                reason: String::from("refused by the fake"),
            });
        }
        self.put(path, text);
        Ok(())
    }
}
