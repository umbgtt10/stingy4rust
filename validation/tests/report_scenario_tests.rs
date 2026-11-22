// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::report_scenario::ReportScenario;

#[test]
fn report_prints_every_profile_s_figures_and_exits_0() {
    // Arrange & Act
    let run = ReportScenario::report();

    // Assert
    assert_eq!(run.exit_code(), Some(0), "{}", run.stderr());
    for line in [
        "profile three-blocks: measured",
        "profile freed-elsewhere: measured",
        "profile nested: measured",
        "profile growing: measured",
        "profile outside: measured",
        "  scopes.grower.reallocations                   4",
    ] {
        assert!(
            run.stdout().contains(line),
            "{line} missing from {}",
            run.stdout()
        );
    }
}
