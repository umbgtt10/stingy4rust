// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

use crate::budget::budget::Budget;
use crate::budget::check_outcome::CheckOutcome;
use crate::budget::figure_verdict::FigureVerdict;
use crate::budget::measured_profile::MeasuredProfile;
use crate::budget::profile_budget::ProfileBudget;
use crate::budget::profile_check::ProfileCheck;
use crate::budget::verdict::Verdict;

// A run's figures held against a target's budget, every profile and every
// figure: each measured figure within its budget, a gain, over it or
// unbudgeted; each budgeted one not measured missing -- whole profiles too.
// With one profile named, it alone is compared. Nothing goes unmeasured by
// omission.
pub struct Comparison;

impl Comparison {
    #[must_use]
    pub fn of(
        budget: &Budget,
        measured: &[MeasuredProfile],
        target: &str,
        profile: Option<&str>,
    ) -> CheckOutcome {
        let budgeted = budget.targets.get(target);
        let chosen = |name: &str| profile.is_none_or(|wanted| wanted == name);
        let mut profiles: Vec<ProfileCheck> = measured
            .iter()
            .filter(|run| chosen(&run.name))
            .map(|run| Self::profile(run, budgeted.and_then(|profiles| profiles.get(&run.name))))
            .collect();
        for (name, profile_budget) in budgeted.into_iter().flatten() {
            if chosen(name) && !measured.iter().any(|run| &run.name == name) {
                profiles.push(ProfileCheck {
                    name: name.clone(),
                    figures: Self::not_measured(profile_budget, &[]),
                });
            }
        }
        CheckOutcome {
            target: String::from(target),
            profiles,
        }
    }

    fn figure(path: &str, budget: Option<&ProfileBudget>, measured: Option<u64>) -> FigureVerdict {
        let budgeted = budget.and_then(|budget| budget.figures.get(path).copied());
        let allowed = budgeted.map(|value| {
            budget
                .and_then(|budget| budget.tolerances.get(path))
                .map_or(value, |tolerance| tolerance.allowed(value))
        });
        let verdict = match (budgeted, measured, allowed) {
            (None, _, _) => Verdict::Unbudgeted,
            (Some(_), None, _) => Verdict::Missing,
            (Some(value), Some(figure), allowed) => {
                if figure > allowed.unwrap_or(value) {
                    Verdict::Over
                } else if figure < value {
                    Verdict::Gain
                } else {
                    Verdict::Within
                }
            }
        };
        FigureVerdict {
            path: String::from(path),
            budget: budgeted,
            measured,
            allowed,
            verdict,
        }
    }

    // The budgeted figures a run did not measure, in the budget's order.
    fn not_measured(budget: &ProfileBudget, measured: &[String]) -> Vec<FigureVerdict> {
        budget
            .figures
            .keys()
            .filter(|path| !measured.contains(path))
            .map(|path| Self::figure(path, Some(budget), None))
            .collect()
    }

    fn profile(run: &MeasuredProfile, budget: Option<&ProfileBudget>) -> ProfileCheck {
        let mut figures: Vec<FigureVerdict> = run
            .figures
            .iter()
            .map(|figure| Self::figure(&figure.path, budget, Some(figure.value)))
            .collect();
        if let Some(budget) = budget {
            let paths: Vec<String> = run
                .figures
                .iter()
                .map(|figure| figure.path.clone())
                .collect();
            figures.extend(Self::not_measured(budget, &paths));
        }
        ProfileCheck {
            name: run.name.clone(),
            figures,
        }
    }
}
