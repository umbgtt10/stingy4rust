// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::collections::BTreeMap;
use alloc::string::String;

use crate::budget::tolerance::Tolerance;

// One profile's budget on one target: each figure's budget by its path, and
// the tolerance of the few figures that carry one.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ProfileBudget {
    pub figures: BTreeMap<String, u64>,
    pub tolerances: BTreeMap<String, Tolerance>,
}
