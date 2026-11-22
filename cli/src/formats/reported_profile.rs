// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::budget::measured_profile::MeasuredProfile;

// One profile of a report as the subcommand reads it: the status its run
// ended in, and its name and figures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportedProfile {
    pub status: String,
    pub measured: MeasuredProfile,
}
