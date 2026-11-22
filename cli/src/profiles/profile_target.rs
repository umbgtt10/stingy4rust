// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// The package whose `stingy` test target holds a project's profiles, with the
// workspace it is in and the directory cargo builds into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileTarget {
    pub package: String,
    pub workspace_root: String,
    pub target_directory: String,
}
