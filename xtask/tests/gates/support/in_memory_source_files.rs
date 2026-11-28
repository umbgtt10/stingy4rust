// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serge4rust::gates::boundary::source_files::SourceFiles;
use serge4rust::gates::boundary::source_read_failure::SourceReadFailure;
use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

// The seam the unsafe gate reads source through, serge4rust's, faked once so
// the gate's tests share one fake: files held in memory, and a directory it was
// not given is one it cannot list.
pub struct InMemorySourceFiles {
    files: BTreeMap<PathBuf, String>,
}

impl InMemorySourceFiles {
    pub const fn new() -> Self {
        Self {
            files: BTreeMap::new(),
        }
    }

    pub fn with_file(mut self, path: &str, contents: &str) -> Self {
        self.files
            .insert(PathBuf::from(path), String::from(contents));
        self
    }
}

impl Default for InMemorySourceFiles {
    fn default() -> Self {
        Self::new()
    }
}

impl SourceFiles for InMemorySourceFiles {
    fn read(&self, file: &Path) -> Result<String, SourceReadFailure> {
        self.files
            .get(file)
            .cloned()
            .ok_or_else(|| SourceReadFailure {
                path: file.to_path_buf(),
                reason: String::from("not held"),
            })
    }

    fn rust_files(&self, dir: &Path) -> Result<Vec<PathBuf>, SourceReadFailure> {
        let files: Vec<PathBuf> = self
            .files
            .keys()
            .filter(|path| path.starts_with(dir))
            .cloned()
            .collect();
        if files.is_empty() {
            return Err(SourceReadFailure {
                path: dir.to_path_buf(),
                reason: String::from("not held"),
            });
        }
        Ok(files)
    }
}
