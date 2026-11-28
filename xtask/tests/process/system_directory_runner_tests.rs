// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use tempfile::TempDir;
use xtask::process::directory_runner::DirectoryRunner;
use xtask::process::system_directory_runner::SystemDirectoryRunner;

#[test]
fn run_streaming_in_a_directory_runs_the_program_there() {
    // Arrange
    let runner = SystemDirectoryRunner::new();
    let empty = TempDir::new().expect("a scratch directory");
    let args = vec![
        String::from("metadata"),
        String::from("--format-version"),
        String::from("1"),
        String::from("--no-deps"),
    ];

    // Act
    let outside = runner
        .run_streaming_in(&empty.path().to_string_lossy(), "cargo", &args)
        .expect("cargo runs");

    // Assert
    assert_ne!(outside, Some(0));
    assert_eq!(
        runner
            .run_streaming_in(env!("CARGO_MANIFEST_DIR"), "cargo", &args)
            .expect("cargo runs"),
        Some(0)
    );
}
