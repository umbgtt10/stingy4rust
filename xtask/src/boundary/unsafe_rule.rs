// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

// Where `unsafe` may appear: the directories scanned, every `.rs` file beneath
// them, and the files among them allowed to hold it -- the allocators', which
// cannot be written without it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsafeRule {
    pub source_dirs: Vec<PathBuf>,
    pub allowed_files: Vec<PathBuf>,
}
