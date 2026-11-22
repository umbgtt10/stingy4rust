// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::report::profile_report::ProfileReport;

// What a profile target writes: every profile it ran, in the order run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    pub profiles: Vec<ProfileReport>,
}
