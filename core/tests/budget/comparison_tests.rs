// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use core::iter::once;
use stingy4rust::budget::budget::Budget;
use stingy4rust::budget::comparison::Comparison;
use stingy4rust::budget::figure_verdict::FigureVerdict;
use stingy4rust::budget::measured_profile::MeasuredProfile;
use stingy4rust::budget::profile_budget::ProfileBudget;
use stingy4rust::budget::tolerance::Tolerance;
use stingy4rust::budget::verdict::Verdict;
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

fn verdict(
    path: &str,
    budget: Option<u64>,
    measured: Option<u64>,
    allowed: Option<u64>,
    verdict: Verdict,
) -> FigureVerdict {
    FigureVerdict {
        path: String::from(path),
        budget,
        measured,
        allowed,
        verdict,
    }
}

fn verdicts(
    budget: &Budget,
    run: &[MeasuredProfile],
    profile: Option<&str>,
) -> Vec<(String, Vec<FigureVerdict>)> {
    Comparison::of(budget, run, TARGET, profile)
        .profiles
        .into_iter()
        .map(|check| (check.name, check.figures))
        .collect()
}

#[test]
fn of_a_budgeted_figure_not_measured_is_missing() {
    // Arrange
    let budget = budget_of(&[(
        "idle",
        profile_budget(&[("peaks.run", 10), ("heap.allocations", 2)], &[]),
    )]);
    let run = [measured("idle", &[("peaks.run", 10)])];

    // Act
    let checked = verdicts(&budget, &run, None);

    // Assert
    assert_eq!(
        checked[0].1,
        vec![
            verdict("peaks.run", Some(10), Some(10), Some(10), Verdict::Within),
            verdict("heap.allocations", Some(2), None, Some(2), Verdict::Missing),
        ]
    );
}

#[test]
fn of_a_budgeted_profile_not_run_is_missing() {
    // Arrange
    let budget = budget_of(&[
        ("idle", profile_budget(&[("peaks.run", 10)], &[])),
        ("busy", profile_budget(&[("peaks.run", 50)], &[])),
    ]);
    let run = [measured("idle", &[("peaks.run", 10)])];

    // Act
    let checked = verdicts(&budget, &run, None);

    // Assert
    assert_eq!(checked.len(), 2);
    assert_eq!(checked[1].0, "busy");
    assert_eq!(
        checked[1].1,
        vec![verdict(
            "peaks.run",
            Some(50),
            None,
            Some(50),
            Verdict::Missing
        )]
    );
}

#[test]
fn of_a_figure_above_its_budget_is_over() {
    // Arrange
    let budget = budget_of(&[("idle", profile_budget(&[("peaks.run", 10)], &[]))]);
    let run = [measured("idle", &[("peaks.run", 11)])];

    // Act
    let checked = verdicts(&budget, &run, None);

    // Assert
    assert_eq!(
        checked[0].1,
        vec![verdict(
            "peaks.run",
            Some(10),
            Some(11),
            Some(10),
            Verdict::Over
        )]
    );
}

#[test]
fn of_a_figure_above_within_its_tolerance_is_within() {
    // Arrange
    let budget = budget_of(&[(
        "idle",
        profile_budget(
            &[("peaks.run", 100)],
            &[("peaks.run", Tolerance::new(Some(5), None))],
        ),
    )]);
    let run = [measured("idle", &[("peaks.run", 105)])];

    // Act
    let checked = verdicts(&budget, &run, None);

    // Assert
    assert_eq!(
        checked[0].1,
        vec![verdict(
            "peaks.run",
            Some(100),
            Some(105),
            Some(105),
            Verdict::Within
        )]
    );
}

#[test]
fn of_a_figure_below_its_budget_is_a_gain_and_passes() {
    // Arrange
    let budget = budget_of(&[("idle", profile_budget(&[("peaks.run", 10)], &[]))]);
    let run = [measured("idle", &[("peaks.run", 9)])];

    // Act
    let checked = verdicts(&budget, &run, None);

    // Assert
    assert_eq!(
        checked[0].1,
        vec![verdict(
            "peaks.run",
            Some(10),
            Some(9),
            Some(10),
            Verdict::Gain
        )]
    );
    assert!(!Verdict::Gain.fails());
}

#[test]
fn of_a_figure_equal_to_its_budget_is_within() {
    // Arrange
    let budget = budget_of(&[("idle", profile_budget(&[("peaks.run", 10)], &[]))]);
    let run = [measured("idle", &[("peaks.run", 10)])];

    // Act
    let checked = verdicts(&budget, &run, None);

    // Assert
    assert_eq!(
        checked,
        vec![(
            String::from("idle"),
            vec![verdict(
                "peaks.run",
                Some(10),
                Some(10),
                Some(10),
                Verdict::Within
            )]
        )]
    );
}

#[test]
fn of_a_figure_with_no_budget_is_unbudgeted() {
    // Arrange
    let budget = budget_of(&[("idle", profile_budget(&[("peaks.run", 10)], &[]))]);
    let run = [measured(
        "idle",
        &[("peaks.run", 10), ("heap.allocations", 3)],
    )];

    // Act
    let checked = verdicts(&budget, &run, None);

    // Assert
    assert_eq!(
        checked[0].1[1],
        verdict("heap.allocations", None, Some(3), None, Verdict::Unbudgeted)
    );
}

#[test]
fn of_a_profile_with_no_budget_is_unbudgeted() {
    // Arrange
    let budget = budget_of(&[("idle", profile_budget(&[("peaks.run", 10)], &[]))]);
    let run = [
        measured("idle", &[("peaks.run", 10)]),
        measured("busy", &[("peaks.run", 50)]),
    ];

    // Act
    let checked = verdicts(&budget, &run, None);

    // Assert
    assert_eq!(
        checked[1],
        (
            String::from("busy"),
            vec![verdict(
                "peaks.run",
                None,
                Some(50),
                None,
                Verdict::Unbudgeted
            )]
        )
    );
}

#[test]
fn of_a_target_with_no_budget_is_unbudgeted() {
    // Arrange
    let budget = Budget::default();
    let run = [measured("idle", &[("peaks.run", 10)])];

    // Act
    let outcome = Comparison::of(&budget, &run, TARGET, None);

    // Assert
    assert_eq!(outcome.target, TARGET);
    assert_eq!(
        outcome.profiles[0].figures,
        vec![verdict(
            "peaks.run",
            None,
            Some(10),
            None,
            Verdict::Unbudgeted
        )]
    );
}

#[test]
fn of_one_profile_named_compares_it_alone() {
    // Arrange
    let budget = budget_of(&[
        ("idle", profile_budget(&[("peaks.run", 10)], &[])),
        ("busy", profile_budget(&[("peaks.run", 50)], &[])),
    ]);
    let run = [measured("busy", &[("peaks.run", 50)])];

    // Act
    let checked = verdicts(&budget, &run, Some("busy"));

    // Assert
    assert_eq!(
        checked,
        vec![(
            String::from("busy"),
            vec![verdict(
                "peaks.run",
                Some(50),
                Some(50),
                Some(50),
                Verdict::Within
            )]
        )]
    );
}
