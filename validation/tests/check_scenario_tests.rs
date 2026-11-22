// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::budget_copy::BudgetCopy;
use validation::check_scenario::CheckScenario;

#[test]
fn check_of_a_budget_lowered_by_one_exits_1_over() {
    // Arrange
    let budget = BudgetCopy::of_fixture().with_figure("three-blocks", "heap.allocations", Some(2));

    // Act
    let run = CheckScenario::check(&budget);

    // Assert
    assert_eq!(run.exit_code(), Some(1), "{}", run.stderr());
    assert!(
        run.stdout().lines().any(|line| line.split_whitespace().eq([
            "heap.allocations",
            "2",
            "3",
            "+1",
            "over"
        ])),
        "{}",
        run.stdout()
    );
}

#[test]
fn check_of_a_budget_missing_a_figure_exits_1_unbudgeted() {
    // Arrange
    let budget = BudgetCopy::of_fixture().with_figure("three-blocks", "peaks.run", None);

    // Act
    let run = CheckScenario::check(&budget);

    // Assert
    assert_eq!(run.exit_code(), Some(1), "{}", run.stderr());
    assert!(run.stdout().contains("1 unbudgeted"), "{}", run.stdout());
}

#[test]
fn check_of_a_budget_raised_by_one_exits_0_with_a_gain() {
    // Arrange
    let budget = BudgetCopy::of_fixture().with_figure("three-blocks", "heap.allocations", Some(4));

    // Act
    let run = CheckScenario::check(&budget);

    // Assert
    assert_eq!(run.exit_code(), Some(0), "{}", run.stderr());
    assert!(
        run.stdout()
            .contains(", 1 gain, 0 over, 0 unbudgeted, 0 missing"),
        "{}",
        run.stdout()
    );
    assert!(
        run.stdout().contains("a gain is kept by recording it"),
        "{}",
        run.stdout()
    );
}

#[test]
fn check_within_the_fixture_s_budget_exits_0() {
    // Arrange
    let budget = BudgetCopy::of_fixture();

    // Act
    let run = CheckScenario::check(&budget);

    // Assert
    assert_eq!(
        run.exit_code(),
        Some(0),
        "{}\n{}",
        run.stdout(),
        run.stderr()
    );
    assert!(
        run.stdout()
            .contains(" within, 0 gains, 0 over, 0 unbudgeted, 0 missing"),
        "{}",
        run.stdout()
    );
}
