// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::formats::reported_profile::ReportedProfile;

// A run of a profile target that measured every profile: the report as it
// was written, and its profiles as read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Measurement {
    pub report: String,
    pub profiles: Vec<ReportedProfile>,
}
