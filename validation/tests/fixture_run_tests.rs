// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::fixture_run::FixtureRun;

#[test]
fn figure_of_a_path_the_report_lacks_is_none() {
    // Arrange
    let run = FixtureRun::of("measured");

    // Act
    let figure = run.figure("three-blocks", "scopes.nobody.peak");

    // Assert
    assert_eq!(figure, None);
    assert_eq!(run.figure("three-blocks", "peaks.run"), Some(192));
}

#[test]
fn of_the_measured_fixture_exits_0_with_every_profile_reported() {
    // Arrange & Act
    let run = FixtureRun::of("measured");

    // Assert
    assert_eq!(run.exit_code(), Some(0));
    for profile in [
        "three-blocks",
        "freed-elsewhere",
        "nested",
        "growing",
        "outside",
        "wire-and-storage",
    ] {
        assert!(
            run.figure(profile, "heap.allocations").is_some(),
            "{profile} missing from {}\n{}",
            run.report(),
            run.log()
        );
    }
}
