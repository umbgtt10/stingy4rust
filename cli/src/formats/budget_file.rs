// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::collections::BTreeMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::from_str;
use serde_json::to_string_pretty;
use stingy4rust::budget::budget::Budget;
use stingy4rust::budget::profile_budget::ProfileBudget;
use stingy4rust::budget::tolerance::Tolerance;

use crate::unable::Unable;

const KIND: &str = "budget";
const VERSION: u64 = 1;

#[derive(Serialize, Deserialize)]
struct BudgetDocument {
    pub stingy4rust: String,
    pub version: u64,
    pub targets: BTreeMap<String, BTreeMap<String, BudgetProfileRecord>>,
}

#[derive(Serialize, Deserialize)]
struct BudgetProfileRecord {
    pub figures: BTreeMap<String, u64>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub tolerances: BTreeMap<String, BudgetToleranceRecord>,
}

#[derive(Serialize, Deserialize)]
struct BudgetToleranceRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub absolute: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relative: Option<f64>,
}

// The budget file: read into the core's budget, refusing a document of
// another kind or version; written with every key sorted and two spaces a
// level, so the same budget is the same bytes and a change reads as a diff.
pub struct BudgetFile;

impl BudgetFile {
    pub fn read(path: &str, text: &str) -> Result<Budget, Unable> {
        let unreadable = |reason: String| Unable::UnreadableBudget {
            path: String::from(path),
            reason,
        };
        let document: BudgetDocument =
            from_str(text).map_err(|error| unreadable(error.to_string()))?;
        if document.stingy4rust != KIND {
            return Err(unreadable(format!(
                "it is a {}, not a budget",
                document.stingy4rust
            )));
        }
        if document.version != VERSION {
            return Err(unreadable(format!(
                "it is version {}; this stingy4rust reads version {VERSION}",
                document.version
            )));
        }
        Ok(Budget {
            targets: document
                .targets
                .into_iter()
                .map(|(target, profiles)| {
                    let profiles = profiles
                        .into_iter()
                        .map(|(name, profile)| (name, Self::profile(profile)))
                        .collect();
                    (target, profiles)
                })
                .collect(),
        })
    }

    #[must_use]
    pub fn write(budget: &Budget) -> String {
        let document = BudgetDocument {
            stingy4rust: String::from(KIND),
            version: VERSION,
            targets: budget
                .targets
                .iter()
                .map(|(target, profiles)| {
                    let profiles = profiles
                        .iter()
                        .map(|(name, profile)| (name.clone(), Self::record(profile)))
                        .collect();
                    (target.clone(), profiles)
                })
                .collect(),
        };
        let mut text = to_string_pretty(&document).expect("a budget always writes as JSON");
        text.push('\n');
        text
    }

    fn profile(record: BudgetProfileRecord) -> ProfileBudget {
        ProfileBudget {
            figures: record.figures,
            tolerances: record
                .tolerances
                .into_iter()
                .map(|(path, tolerance)| {
                    (path, Tolerance::new(tolerance.absolute, tolerance.relative))
                })
                .collect(),
        }
    }

    fn record(profile: &ProfileBudget) -> BudgetProfileRecord {
        BudgetProfileRecord {
            figures: profile.figures.clone(),
            tolerances: profile
                .tolerances
                .iter()
                .map(|(path, tolerance)| {
                    (
                        path.clone(),
                        BudgetToleranceRecord {
                            absolute: tolerance.absolute(),
                            relative: tolerance.relative(),
                        },
                    )
                })
                .collect(),
        }
    }
}
