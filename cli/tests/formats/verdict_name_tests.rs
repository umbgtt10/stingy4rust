// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::budget::verdict::Verdict;
use stingy4rust_cli::formats::verdict_name::VerdictName;

#[test]
fn of_each_verdict_is_its_name_in_lower_case() {
    // Arrange & Act
    let names = [
        VerdictName::of(Verdict::Within),
        VerdictName::of(Verdict::Gain),
        VerdictName::of(Verdict::Over),
        VerdictName::of(Verdict::Unbudgeted),
        VerdictName::of(Verdict::Missing),
    ];

    // Assert
    assert_eq!(names, ["within", "gain", "over", "unbudgeted", "missing"]);
}
