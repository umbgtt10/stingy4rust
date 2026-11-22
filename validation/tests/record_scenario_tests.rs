// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::record_scenario::RecordScenario;

#[test]
fn record_then_check_exits_0_both_times() {
    // Arrange & Act
    let runs = RecordScenario::record_then_check();

    // Assert
    assert_eq!(runs.record.exit_code(), Some(0), "{}", runs.record.stderr());
    assert!(
        runs.record.stdout().contains("unbudgeted"),
        "{}",
        runs.record.stdout()
    );
    assert_eq!(runs.check.exit_code(), Some(0), "{}", runs.check.stderr());
    assert!(
        runs.check.stdout().contains("0 unbudgeted"),
        "{}",
        runs.check.stdout()
    );
}
