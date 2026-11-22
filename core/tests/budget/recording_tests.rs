// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::iter::once;
use stingy4rust::budget::budget::Budget;
use stingy4rust::budget::measured_profile::MeasuredProfile;
use stingy4rust::budget::profile_budget::ProfileBudget;
use stingy4rust::budget::recording::Recording;
use stingy4rust::budget::tolerance::Tolerance;
use stingy4rust::figures::figure::Figure;

const TARGET: &str = "x86_64-unknown-linux-gnu";

fn budget_of(profiles: &[(&str, ProfileBudget)]) -> Budget {
    let profiles = profiles
        .iter()
        .map(|(name, budget)| (String::from(*name), budget.clone()))
        .collect();
    Budget {
        targets: once((String::from(TARGET), profiles)).collect(),
    }
}

fn figures(figures: &[(&str, u64)]) -> Vec<Figure> {
    figures
        .iter()
        .map(|(path, value)| Figure {
            path: String::from(*path),
            value: *value,
        })
        .collect()
}

fn measured(name: &str, measured: &[(&str, u64)]) -> MeasuredProfile {
    MeasuredProfile {
        name: String::from(name),
        figures: figures(measured),
    }
}

fn profile_budget(figures: &[(&str, u64)], tolerances: &[(&str, Tolerance)]) -> ProfileBudget {
    ProfileBudget {
        figures: figures
            .iter()
            .map(|(path, value)| (String::from(*path), *value))
            .collect(),
        tolerances: tolerances
            .iter()
            .map(|(path, tolerance)| (String::from(*path), *tolerance))
            .collect(),
    }
}

#[test]
fn of_a_report_drops_the_tolerances_of_figures_gone() {
    // Arrange
    let budget = budget_of(&[(
        "idle",
        profile_budget(
            &[("peaks.run", 10), ("scopes.old.peak", 4)],
            &[("scopes.old.peak", Tolerance::new(Some(1), None))],
        ),
    )]);
    let run = [measured("idle", &[("peaks.run", 12)])];

    // Act
    let recorded = Recording::of(&budget, &run, TARGET, None);

    // Assert
    assert_eq!(
        recorded,
        budget_of(&[("idle", profile_budget(&[("peaks.run", 12)], &[]))])
    );
}

#[test]
fn of_a_report_keeps_the_tolerances_of_figures_still_measured() {
    // Arrange
    let tolerance = Tolerance::new(Some(4096), None);
    let budget = budget_of(&[(
        "idle",
        profile_budget(
            &[("storage.log.bytes_on_disk", 8192)],
            &[("storage.log.bytes_on_disk", tolerance)],
        ),
    )]);
    let run = [measured("idle", &[("storage.log.bytes_on_disk", 12288)])];

    // Act
    let recorded = Recording::of(&budget, &run, TARGET, None);

    // Assert
    assert_eq!(
        recorded,
        budget_of(&[(
            "idle",
            profile_budget(
                &[("storage.log.bytes_on_disk", 12288)],
                &[("storage.log.bytes_on_disk", tolerance)]
            ),
        )])
    );
}

#[test]
fn of_a_report_leaves_other_targets_as_they_were() {
    // Arrange
    let mut budget = budget_of(&[("idle", profile_budget(&[("peaks.run", 10)], &[]))]);
    let other =
        budget_of(&[("idle", profile_budget(&[("peaks.run", 7)], &[]))]).targets[TARGET].clone();
    budget
        .targets
        .insert(String::from("thumbv7em-none-eabihf"), other.clone());
    let run = [measured("idle", &[("peaks.run", 12)])];

    // Act
    let recorded = Recording::of(&budget, &run, TARGET, None);

    // Assert
    assert_eq!(recorded.targets["thumbv7em-none-eabihf"], other);
    assert_eq!(recorded.targets[TARGET]["idle"].figures["peaks.run"], 12);
}

#[test]
fn of_a_report_writes_its_figures_as_the_target_s_budget() {
    // Arrange
    let budget = budget_of(&[
        ("idle", profile_budget(&[("peaks.run", 10)], &[])),
        ("retired", profile_budget(&[("peaks.run", 99)], &[])),
    ]);
    let run = [
        measured("idle", &[("peaks.run", 12), ("heap.allocations", 3)]),
        measured("busy", &[("peaks.run", 50)]),
    ];

    // Act
    let recorded = Recording::of(&budget, &run, TARGET, None);

    // Assert
    assert_eq!(
        recorded,
        budget_of(&[
            ("busy", profile_budget(&[("peaks.run", 50)], &[])),
            (
                "idle",
                profile_budget(&[("heap.allocations", 3), ("peaks.run", 12)], &[])
            ),
        ])
    );
}

#[test]
fn of_one_profile_named_replaces_it_alone() {
    // Arrange
    let budget = budget_of(&[
        ("idle", profile_budget(&[("peaks.run", 10)], &[])),
        ("busy", profile_budget(&[("peaks.run", 50)], &[])),
    ]);
    let run = [measured("busy", &[("peaks.run", 40)])];

    // Act
    let recorded = Recording::of(&budget, &run, TARGET, Some("busy"));

    // Assert
    assert_eq!(
        recorded,
        budget_of(&[
            ("busy", profile_budget(&[("peaks.run", 40)], &[])),
            ("idle", profile_budget(&[("peaks.run", 10)], &[])),
        ])
    );
}
