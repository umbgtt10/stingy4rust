// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Serialize;
use serde_json::to_string;
use stingy4rust::budget::check_outcome::CheckOutcome;

use crate::formats::verdict_name::VerdictName;

const VERSION: u64 = 1;

#[derive(Serialize)]
struct CheckDocument<'a> {
    pub stingy4rust: &'static str,
    pub version: u64,
    pub target: &'a str,
    pub verdict: &'static str,
    pub profiles: Vec<CheckProfileRecord<'a>>,
}

#[derive(Serialize)]
struct CheckProfileRecord<'a> {
    pub name: &'a str,
    pub figures: Vec<CheckFigureRecord<'a>>,
}

#[derive(Serialize)]
struct CheckFigureRecord<'a> {
    pub path: &'a str,
    pub budget: Option<u64>,
    pub measured: Option<u64>,
    pub allowed: Option<u64>,
    pub verdict: &'static str,
}

// A check as one line of JSON, as the family's tools write theirs: the
// target, the verdict of the whole -- over if any figure fails -- and every
// figure's budget, measurement, allowance and verdict.
pub struct CheckJson;

impl CheckJson {
    #[must_use]
    pub fn write(outcome: &CheckOutcome) -> String {
        let fails = outcome
            .profiles
            .iter()
            .flat_map(|profile| &profile.figures)
            .any(|figure| figure.verdict.fails());
        let document = CheckDocument {
            stingy4rust: "check",
            version: VERSION,
            target: &outcome.target,
            verdict: if fails { "over" } else { "within" },
            profiles: outcome
                .profiles
                .iter()
                .map(|profile| CheckProfileRecord {
                    name: &profile.name,
                    figures: profile
                        .figures
                        .iter()
                        .map(|figure| CheckFigureRecord {
                            path: &figure.path,
                            budget: figure.budget,
                            measured: figure.measured,
                            allowed: figure.allowed,
                            verdict: VerdictName::of(figure.verdict),
                        })
                        .collect(),
                })
                .collect(),
        };
        let mut text = to_string(&document).expect("a check always writes as JSON");
        text.push('\n');
        text
    }
}
