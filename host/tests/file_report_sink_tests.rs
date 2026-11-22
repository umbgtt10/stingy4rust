// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs::read_to_string;
use stingy4rust_host::file_report_sink::FileReportSink;
use stingy4rust_host::report_sink::ReportSink;
use tempfile::TempDir;

#[test]
fn write_into_a_directory_not_there_yet_makes_it() {
    // Arrange
    let scratch = TempDir::new().expect("a scratch directory");
    let path = scratch
        .path()
        .join("target")
        .join("stingy4rust")
        .join("measured.json");

    // Act
    let outcome = FileReportSink.write(&path.to_string_lossy(), "{}\n");

    // Assert
    assert_eq!(outcome, Ok(()));
    assert_eq!(read_to_string(&path).expect("the report"), "{}\n");
}

#[test]
fn write_puts_the_text_at_the_path() {
    // Arrange
    let scratch = TempDir::new().expect("a scratch directory");
    let path = scratch.path().join("report.json");

    // Act
    let outcome = FileReportSink.write(&path.to_string_lossy(), "{\"version\": 1}\n");

    // Assert
    assert_eq!(outcome, Ok(()));
    assert_eq!(
        read_to_string(&path).expect("the report"),
        "{\"version\": 1}\n"
    );
}

#[test]
fn write_to_a_path_that_is_a_directory_fails_naming_it() {
    // Arrange
    let scratch = TempDir::new().expect("a scratch directory");
    let path = scratch.path().to_string_lossy().into_owned();

    // Act
    let outcome = FileReportSink.write(&path, "{}\n");

    // Assert
    let failure = outcome.expect_err("a directory is not a file");
    assert_eq!(failure.path, path);
    assert_ne!(failure.reason, "");
}
