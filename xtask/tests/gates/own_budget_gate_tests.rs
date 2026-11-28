// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::gates::support::fake_directory_runner::FakeDirectoryRunner;
use serge4rust::gates::gate::Gate;
use serge4rust::gates::gate_failure::GateFailure;
use xtask::gates::own_budget_gate::OwnBudgetGate;

fn gate(runner: &FakeDirectoryRunner) -> OwnBudgetGate<'_> {
    OwnBudgetGate::new(
        runner,
        String::from("/repo/Cargo.toml"),
        String::from("/repo/fixture"),
        String::from("measured"),
    )
}

#[test]
fn label_names_the_own_budget_gate() {
    // Arrange
    let runner = FakeDirectoryRunner::new();

    // Act
    let label = gate(&runner).label();

    // Assert
    assert_eq!(label, "Own budget");
}

#[test]
fn run_builds_the_subcommand_from_source_and_checks_the_fixture_in_its_directory() {
    // Arrange
    let runner = FakeDirectoryRunner::new();

    // Act
    let _ = gate(&runner).run();

    // Assert
    assert_eq!(runner.programs(), vec![String::from("cargo")]);
    assert_eq!(runner.directories(), vec![String::from("/repo/fixture")]);
    assert_eq!(
        runner.calls()[0],
        [
            "run",
            "--quiet",
            "--release",
            "--manifest-path",
            "/repo/Cargo.toml",
            "--bin",
            "cargo-stingy4rust",
            "--",
            "stingy4rust",
            "check",
            "--package",
            "measured",
        ]
        .map(String::from)
        .to_vec()
    );
}

#[test]
fn run_ended_by_a_signal_reports_the_missing_code() {
    // Arrange
    let runner = FakeDirectoryRunner::new().with_streaming_code(None);

    // Act
    let outcome = gate(&runner).run();

    // Assert
    assert_eq!(outcome, Err(GateFailure::CouldNotRun { exit_code: None }));
}

#[test]
fn run_over_its_budget_fails_naming_the_budget() {
    // Arrange
    let runner = FakeDirectoryRunner::new().with_streaming_code(Some(1));

    // Act
    let outcome = gate(&runner).run();

    // Assert
    assert_eq!(
        outcome,
        Err(GateFailure::Own {
            reason: String::from(
                "the fixture's figures moved from /repo/fixture/stingy4rust-budget.json"
            )
        })
    );
}

#[test]
fn run_unable_to_measure_could_not_run() {
    // Arrange
    let runner = FakeDirectoryRunner::new().with_streaming_code(Some(2));

    // Act
    let outcome = gate(&runner).run();

    // Assert
    assert_eq!(
        outcome,
        Err(GateFailure::CouldNotRun { exit_code: Some(2) })
    );
}

#[test]
fn run_whose_cargo_cannot_start_fails_to_launch() {
    // Arrange
    let runner = FakeDirectoryRunner::new().with_launch_failure();

    // Act
    let outcome = gate(&runner).run();

    // Assert
    assert!(
        matches!(outcome, Err(GateFailure::Launch(_))),
        "{outcome:?}"
    );
}

#[test]
fn run_whose_subcommand_does_not_build_could_not_run() {
    // Arrange
    let runner = FakeDirectoryRunner::new().with_streaming_code(Some(101));

    // Act
    let outcome = gate(&runner).run();

    // Assert
    assert_eq!(
        outcome,
        Err(GateFailure::CouldNotRun {
            exit_code: Some(101)
        })
    );
}

#[test]
fn run_within_its_budget_passes() {
    // Arrange
    let runner = FakeDirectoryRunner::new().with_streaming_code(Some(0));

    // Act
    let outcome = gate(&runner).run();

    // Assert
    assert_eq!(outcome, Ok(()));
    assert_eq!(runner.programs(), vec![String::from("cargo")]);
}
