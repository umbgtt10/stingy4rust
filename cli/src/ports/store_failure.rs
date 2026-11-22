// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// A file that could not be read, removed or written: which, and why not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreFailure {
    pub path: String,
    pub reason: String,
}
