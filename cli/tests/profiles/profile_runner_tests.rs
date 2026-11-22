// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fake_command_runner::FakeCommandRunner;
use crate::support::fake_file_store::FakeFileStore;
use crate::support::sample::REPORT_PATH;
use crate::support::sample::report;
use stingy4rust_cli::ports::invocation::Invocation;
use stingy4rust_cli::profiles::profile_runner::ProfileRunner;
use stingy4rust_cli::profiles::profile_target::ProfileTarget;
use stingy4rust_cli::unable::Unable;

fn target() -> ProfileTarget {
    ProfileTarget {
        package: String::from("measured"),
        workspace_root: String::from("/w"),
        target_directory: String::from("/w/target"),
    }
}

fn test_command() -> Vec<String> {
    [
        "test",
        "--release",
        "--package",
        "measured",
        "--test",
        "stingy",
    ]
    .map(String::from)
    .to_vec()
}

#[test]
fn measure_of_a_build_that_fails_is_unable() {
    // Arrange
    let store = FakeFileStore::new();
    let runner = FakeCommandRunner::new().answering("cargo test", Some(101), "");

    // Act
    let measured = ProfileRunner::new(&runner, &store).measure(&target(), None, None);

    // Assert
    assert_eq!(
        measured.map(|measurement| measurement.report),
        Err(Unable::Failed {
            command: String::from("cargo test --release --package measured --test stingy"),
            code: Some(101),
        })
    );
}

#[test]
fn measure_of_a_run_not_measured_is_unable_naming_its_status() {
    // Arrange
    let store = FakeFileStore::new();
    let runner = FakeCommandRunner::new()
        .answering("cargo test", Some(101), "")
        .writing_report(&store, REPORT_PATH, &report("foreign-thread", 3));

    // Act
    let measured = ProfileRunner::new(&runner, &store).measure(&target(), None, None);

    // Assert
    assert_eq!(
        measured.map(|measurement| measurement.report),
        Err(Unable::NotMeasured {
            profile: String::from("three-blocks"),
            status: String::from("foreign-thread"),
        })
    );
}

#[test]
fn measure_removes_a_report_left_by_an_earlier_run() {
    // Arrange
    let store = FakeFileStore::new().with(REPORT_PATH, &report("measured", 3));
    let runner = FakeCommandRunner::new().answering("cargo test", Some(101), "");

    // Act
    let measured = ProfileRunner::new(&runner, &store).measure(&target(), None, None);

    // Assert
    assert!(
        matches!(measured, Err(Unable::Failed { .. })),
        "{measured:?}"
    );
    assert_eq!(store.file(REPORT_PATH), None);
}

#[test]
fn measure_runs_cargo_test_in_release_with_the_report_path() {
    // Arrange
    let store = FakeFileStore::new();
    let text = report("measured", 3);
    let runner = FakeCommandRunner::new()
        .answering("cargo test", Some(0), "")
        .writing_report(&store, REPORT_PATH, &text);

    // Act
    let measured = ProfileRunner::new(&runner, &store).measure(&target(), None, None);

    // Assert
    let measurement = measured.expect("a measurement");
    assert_eq!(measurement.report, text);
    assert_eq!(measurement.profiles[0].measured.name, "three-blocks");
    assert_eq!(
        runner.calls(),
        vec![Invocation {
            program: String::from("cargo"),
            arguments: test_command(),
            directory: Some(String::from("/w")),
            environment: vec![(
                String::from("STINGY4RUST_REPORT"),
                String::from(REPORT_PATH)
            )],
            passes_through: true,
        }]
    );
}

#[test]
fn measure_with_a_profile_named_passes_it() {
    // Arrange
    let store = FakeFileStore::new();
    let runner = FakeCommandRunner::new()
        .answering("cargo test", Some(0), "")
        .writing_report(&store, REPORT_PATH, &report("measured", 3));

    // Act
    let measured = ProfileRunner::new(&runner, &store).measure(&target(), Some("growing"), None);

    // Assert
    assert!(measured.is_ok(), "{measured:?}");
    assert_eq!(
        runner.calls()[0].environment,
        vec![
            (
                String::from("STINGY4RUST_REPORT"),
                String::from(REPORT_PATH)
            ),
            (String::from("STINGY4RUST_PROFILE"), String::from("growing")),
        ]
    );
}

#[test]
fn measure_with_a_target_passes_it_to_cargo() {
    // Arrange
    let store = FakeFileStore::new();
    let runner = FakeCommandRunner::new()
        .answering("cargo test", Some(0), "")
        .writing_report(&store, REPORT_PATH, &report("measured", 3));

    // Act
    let measured =
        ProfileRunner::new(&runner, &store).measure(&target(), None, Some("thumbv7em-none-eabihf"));

    // Assert
    assert!(measured.is_ok(), "{measured:?}");
    let mut expected = test_command();
    expected.extend([
        String::from("--target"),
        String::from("thumbv7em-none-eabihf"),
    ]);
    assert_eq!(runner.calls()[0].arguments, expected);
}
