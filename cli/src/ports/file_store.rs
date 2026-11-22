// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::ports::store_failure::StoreFailure;

// Where the subcommand reads reports and reads and writes budgets: the disk,
// or a test's fake. A file not there is read as none and removed as done.
pub trait FileStore {
    fn read(&self, path: &str) -> Result<Option<String>, StoreFailure>;
    fn remove(&self, path: &str) -> Result<(), StoreFailure>;
    fn write(&self, path: &str, text: &str) -> Result<(), StoreFailure>;
}
