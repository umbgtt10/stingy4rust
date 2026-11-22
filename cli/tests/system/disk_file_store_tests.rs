// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs::read_to_string;
use std::fs::write;
use std::path::Path;
use stingy4rust_cli::ports::file_store::FileStore;
use stingy4rust_cli::system::disk_file_store::DiskFileStore;
use tempfile::TempDir;

fn path_in(scratch: &TempDir, name: &str) -> String {
    scratch.path().join(name).to_string_lossy().into_owned()
}

#[test]
fn read_of_a_file_is_its_text() {
    // Arrange
    let scratch = TempDir::new().expect("a scratch directory");
    let path = path_in(&scratch, "budget.json");
    write(&path, "{}").expect("a file");

    // Act
    let text = DiskFileStore.read(&path);

    // Assert
    assert_eq!(text, Ok(Some(String::from("{}"))));
}

#[test]
fn read_of_a_file_not_there_is_none() {
    // Arrange
    let scratch = TempDir::new().expect("a scratch directory");
    let there = path_in(&scratch, "report.json");
    write(&there, "{}").expect("a file");

    // Act
    let text = DiskFileStore.read(&path_in(&scratch, "budget.json"));

    // Assert
    assert_eq!(text, Ok(None));
    assert_eq!(DiskFileStore.read(&there), Ok(Some(String::from("{}"))));
}

#[test]
fn remove_of_a_file_deletes_it_and_of_one_not_there_is_fine() {
    // Arrange
    let scratch = TempDir::new().expect("a scratch directory");
    let path = path_in(&scratch, "report.json");
    write(&path, "{}").expect("a file");

    // Act
    let removed = [DiskFileStore.remove(&path), DiskFileStore.remove(&path)];

    // Assert
    assert_eq!(removed, [Ok(()), Ok(())]);
    assert!(!Path::new(&path).exists());
}

#[test]
fn write_into_directories_not_there_yet_makes_them() {
    // Arrange
    let scratch = TempDir::new().expect("a scratch directory");
    let path = path_in(&scratch, "a/b/budget.json");

    // Act
    let written = DiskFileStore.write(&path, "{}\n");

    // Assert
    assert_eq!(written, Ok(()));
    assert_eq!(read_to_string(&path).expect("the file"), "{}\n");
}

#[test]
fn write_to_a_directory_fails_naming_it() {
    // Arrange
    let scratch = TempDir::new().expect("a scratch directory");
    let path = scratch.path().to_string_lossy().into_owned();

    // Act
    let written = DiskFileStore.write(&path, "{}");

    // Assert
    let failure = written.expect_err("a directory is not a file");
    assert_eq!(failure.path, path);
    assert_ne!(failure.reason, "");
}
