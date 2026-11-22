// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::collections::BTreeMap;
use alloc::string::String;

use crate::budget::profile_budget::ProfileBudget;

// A project's budgets: by target triple, then by profile -- every key sorted,
// so the same budget is written the same way.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Budget {
    pub targets: BTreeMap<String, BTreeMap<String, ProfileBudget>>,
}
