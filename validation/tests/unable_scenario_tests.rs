// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::unable_scenario::UnableScenario;

#[test]
fn check_of_a_package_whose_profile_starts_a_thread_exits_2_foreign_thread() {
    // Arrange & Act
    let run = UnableScenario::check_of("threaded");

    // Assert
    assert_eq!(run.exit_code(), Some(2), "{}", run.stdout());
    assert!(
        run.stderr()
            .contains("could not be measured: foreign-thread"),
        "{}",
        run.stderr()
    );
}

#[test]
fn check_of_a_package_whose_tables_fill_exits_2_blocks_exhausted() {
    // Arrange & Act
    let run = UnableScenario::check_of("overflowing");

    // Assert
    assert_eq!(run.exit_code(), Some(2), "{}", run.stdout());
    assert!(
        run.stderr()
            .contains("could not be measured: blocks-exhausted"),
        "{}",
        run.stderr()
    );
    assert_eq!(run.stdout(), "");
}

#[test]
fn check_of_a_package_with_no_profile_target_exits_2() {
    // Arrange & Act
    let run = UnableScenario::check_of("unmeasured");

    // Assert
    assert_eq!(run.exit_code(), Some(2), "{}", run.stdout());
    assert!(
        run.stderr()
            .contains("package unmeasured has no test target named stingy"),
        "{}",
        run.stderr()
    );
}

#[test]
fn check_with_a_corrupt_budget_exits_2() {
    // Arrange & Act
    let run = UnableScenario::check_with_a_corrupt_budget();

    // Assert
    assert_eq!(run.exit_code(), Some(2), "{}", run.stdout());
    assert!(
        run.stderr().contains("could not be read"),
        "{}",
        run.stderr()
    );
}
