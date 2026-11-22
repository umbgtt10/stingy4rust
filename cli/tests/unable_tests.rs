// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust_cli::unable::Unable;

#[test]
fn to_string_of_a_run_not_measured_names_the_profile_and_its_status() {
    // Arrange
    let unable = Unable::NotMeasured {
        profile: String::from("three-blocks"),
        status: String::from("blocks-exhausted"),
    };

    // Act
    let text = unable.to_string();

    // Assert
    assert_eq!(
        text,
        "profile three-blocks could not be measured: blocks-exhausted"
    );
}

#[test]
fn to_string_of_each_failure_says_what_could_not_be_done() {
    // Arrange
    let failures = [
        Unable::NoProfileTarget,
        Unable::SeveralProfileTargets(vec![String::from("a"), String::from("b")]),
        Unable::NotAProfileTarget(String::from("tools")),
        Unable::Launch {
            program: String::from("cargo"),
            reason: String::from("not found"),
        },
        Unable::Failed {
            command: String::from("cargo test"),
            code: Some(101),
        },
        Unable::Failed {
            command: String::from("cargo test"),
            code: None,
        },
        Unable::UnreadableMetadata(String::from("expected value")),
        Unable::NoHostTriple,
        Unable::UnreadableReport(String::from("missing field `profiles`")),
        Unable::UnreadableBudget {
            path: String::from("stingy4rust-budget.json"),
            reason: String::from("unknown version 2"),
        },
        Unable::UnwritableBudget {
            path: String::from("stingy4rust-budget.json"),
            reason: String::from("permission denied"),
        },
    ];

    // Act
    let texts: Vec<String> = failures.iter().map(ToString::to_string).collect();

    // Assert
    assert_eq!(
        texts,
        vec![
            "no package of the workspace has a test target named stingy",
            "several packages have a test target named stingy -- name one with --package: a, b",
            "package tools has no test target named stingy",
            "could not start cargo: not found",
            "cargo test failed with exit code 101",
            "cargo test was ended by a signal",
            "cargo metadata could not be read: expected value",
            "rustc -vV named no host",
            "the report could not be read: missing field `profiles`",
            "the budget stingy4rust-budget.json could not be read: unknown version 2",
            "the budget stingy4rust-budget.json could not be written: permission denied",
        ]
    );
}
