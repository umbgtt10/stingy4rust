// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::budget::verdict::Verdict;

#[test]
fn fails_of_each_verdict_is_true_for_over_unbudgeted_and_missing_alone() {
    // Arrange
    let verdicts = [
        Verdict::Within,
        Verdict::Gain,
        Verdict::Over,
        Verdict::Unbudgeted,
        Verdict::Missing,
    ];

    // Act
    let failing = verdicts.map(Verdict::fails);

    // Assert
    assert_eq!(failing, [false, false, true, true, true]);
}
