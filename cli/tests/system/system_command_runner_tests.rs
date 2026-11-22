// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust_cli::ports::command_runner::CommandRunner;
use stingy4rust_cli::ports::invocation::Invocation;
use stingy4rust_cli::system::system_command_runner::SystemCommandRunner;
use stingy4rust_cli::unable::Unable;
use tempfile::TempDir;

fn cargo(arguments: &[&str], directory: Option<String>) -> Invocation {
    Invocation {
        program: String::from("cargo"),
        arguments: arguments
            .iter()
            .map(|argument| String::from(*argument))
            .collect(),
        directory,
        environment: Vec::new(),
        passes_through: false,
    }
}

#[test]
fn run_in_a_directory_runs_the_program_there() {
    // Arrange
    let empty = TempDir::new().expect("a scratch directory");
    let outside = cargo(
        &["metadata", "--format-version", "1", "--no-deps"],
        Some(empty.path().to_string_lossy().into_owned()),
    );

    // Act
    let output = SystemCommandRunner.run(&outside).expect("cargo starts");

    // Assert
    assert_ne!(output.code, Some(0));
    let here = SystemCommandRunner
        .run(&cargo(
            &["metadata", "--format-version", "1", "--no-deps"],
            None,
        ))
        .expect("cargo starts");
    assert_eq!(here.code, Some(0));
}

#[test]
fn run_of_a_program_captures_its_output_and_exit_code() {
    // Arrange
    let invocation = cargo(&["--version"], None);

    // Act
    let output = SystemCommandRunner.run(&invocation).expect("cargo starts");

    // Assert
    assert_eq!(output.code, Some(0));
    assert!(output.stdout.starts_with("cargo "), "{}", output.stdout);
}

#[test]
fn run_of_a_program_that_does_not_exist_is_unable_to_start_it() {
    // Arrange
    let invocation = Invocation {
        program: String::from("stingy4rust-no-such-program"),
        arguments: Vec::new(),
        directory: None,
        environment: Vec::new(),
        passes_through: false,
    };

    // Act
    let output = SystemCommandRunner.run(&invocation);

    // Assert
    assert!(
        matches!(&output, Err(Unable::Launch { program, .. }) if program == "stingy4rust-no-such-program"),
        "{output:?}"
    );
}
