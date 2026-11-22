// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

use crate::budget::profile_check::ProfileCheck;

// A check of one target: every profile compared, those run first in the
// order run, then those budgeted and not run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckOutcome {
    pub target: String,
    pub profiles: Vec<ProfileCheck>,
}
