// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fake_command_runner::FakeCommandRunner;
use crate::support::sample::metadata;
use stingy4rust_cli::profiles::profile_target::ProfileTarget;
use stingy4rust_cli::profiles::target_finder::TargetFinder;
use stingy4rust_cli::unable::Unable;

fn measured() -> ProfileTarget {
    ProfileTarget {
        package: String::from("measured"),
        workspace_root: String::from("/w"),
        target_directory: String::from("/w/target"),
    }
}

#[test]
fn find_of_metadata_that_cannot_be_read_is_unable() {
    // Arrange
    let runner = FakeCommandRunner::new().answering("cargo metadata", Some(0), "not json");

    // Act
    let found = TargetFinder::new(&runner).find(None);

    // Assert
    assert!(
        matches!(found, Err(Unable::UnreadableMetadata(_))),
        "{found:?}"
    );
}

#[test]
fn find_when_cargo_metadata_fails_is_unable() {
    // Arrange
    let runner = FakeCommandRunner::new().answering("cargo metadata", Some(101), "");

    // Act
    let found = TargetFinder::new(&runner).find(None);

    // Assert
    assert_eq!(
        found,
        Err(Unable::Failed {
            command: String::from("cargo metadata --format-version 1 --no-deps"),
            code: Some(101),
        })
    );
}

#[test]
fn find_with_a_package_named_is_it() {
    // Arrange
    let text = metadata(&[("threaded", true), ("measured", true)]);
    let runner = FakeCommandRunner::new().answering("cargo metadata", Some(0), &text);

    // Act
    let found = TargetFinder::new(&runner).find(Some("measured"));

    // Assert
    assert_eq!(found, Ok(measured()));
}

#[test]
fn find_with_a_package_named_that_has_no_stingy_test_fails_naming_it() {
    // Arrange
    let text = metadata(&[("measured", true), ("tools", false)]);
    let runner = FakeCommandRunner::new().answering("cargo metadata", Some(0), &text);

    // Act
    let found = TargetFinder::new(&runner).find(Some("tools"));

    // Assert
    assert_eq!(found, Err(Unable::NotAProfileTarget(String::from("tools"))));
}

#[test]
fn find_with_none_fails_naming_the_target() {
    // Arrange
    let text = metadata(&[("tools", false)]);
    let runner = FakeCommandRunner::new().answering("cargo metadata", Some(0), &text);

    // Act
    let found = TargetFinder::new(&runner).find(None);

    // Assert
    assert_eq!(found, Err(Unable::NoProfileTarget));
}

#[test]
fn find_with_one_package_holding_a_stingy_test_is_it() {
    // Arrange
    let text = metadata(&[("tools", false), ("measured", true)]);
    let runner = FakeCommandRunner::new().answering("cargo metadata", Some(0), &text);

    // Act
    let found = TargetFinder::new(&runner).find(None);

    // Assert
    assert_eq!(found, Ok(measured()));
    assert_eq!(
        runner.calls()[0].arguments,
        vec!["metadata", "--format-version", "1", "--no-deps"]
    );
}

#[test]
fn find_with_two_and_no_package_named_fails_listing_them() {
    // Arrange
    let text = metadata(&[("measured", true), ("threaded", true)]);
    let runner = FakeCommandRunner::new().answering("cargo metadata", Some(0), &text);

    // Act
    let found = TargetFinder::new(&runner).find(None);

    // Assert
    assert_eq!(
        found,
        Err(Unable::SeveralProfileTargets(vec![
            String::from("measured"),
            String::from("threaded"),
        ]))
    );
}
