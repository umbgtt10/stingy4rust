// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::collections::BTreeMap;
use alloc::string::String;

use crate::budget::budget::Budget;
use crate::budget::measured_profile::MeasuredProfile;
use crate::budget::profile_budget::ProfileBudget;

// A run's figures written as a target's budget: every profile run replaces the
// target's budgets -- or, with one profile named, that profile's alone -- each
// keeping the tolerances of its figures still measured and dropping those of
// figures gone. Other targets stay as they were.
pub struct Recording;

impl Recording {
    #[must_use]
    pub fn of(
        budget: &Budget,
        measured: &[MeasuredProfile],
        target: &str,
        profile: Option<&str>,
    ) -> Budget {
        let previous = budget.targets.get(target);
        let recorded = measured
            .iter()
            .filter(|run| profile.is_none_or(|wanted| wanted == run.name))
            .map(|run| {
                let before = previous.and_then(|profiles| profiles.get(&run.name));
                (run.name.clone(), Self::profile(run, before))
            });
        let profiles: BTreeMap<String, ProfileBudget> = match profile {
            None => recorded.collect(),
            Some(_) => previous
                .cloned()
                .unwrap_or_default()
                .into_iter()
                .chain(recorded)
                .collect(),
        };
        let mut targets = budget.targets.clone();
        targets.insert(String::from(target), profiles);
        Budget { targets }
    }

    fn profile(run: &MeasuredProfile, before: Option<&ProfileBudget>) -> ProfileBudget {
        let figures: BTreeMap<String, u64> = run
            .figures
            .iter()
            .map(|figure| (figure.path.clone(), figure.value))
            .collect();
        let tolerances = before
            .map(|before| {
                before
                    .tolerances
                    .iter()
                    .filter(|(path, _)| figures.contains_key(*path))
                    .map(|(path, tolerance)| (path.clone(), *tolerance))
                    .collect()
            })
            .unwrap_or_default();
        ProfileBudget {
            figures,
            tolerances,
        }
    }
}
