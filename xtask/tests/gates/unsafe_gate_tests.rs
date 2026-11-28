// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::gates::support::in_memory_source_files::InMemorySourceFiles;
use serge4rust::gates::boundary::source_read_failure::SourceReadFailure;
use serge4rust::gates::gate::Gate;
use serge4rust::gates::gate_failure::GateFailure;
use std::path::PathBuf;
use xtask::boundary::unsafe_rule::UnsafeRule;
use xtask::gates::unsafe_gate::UnsafeGate;

const ALLOCATOR: &str = "/w/host/src/stingy.rs";
const SAFE: &str = "pub fn f(values: &[u8]) -> usize { values.len() }";
const UNSAFE: &str = "struct S;\nunsafe impl Sync for S {}";

fn rule() -> UnsafeRule {
    UnsafeRule {
        source_dirs: vec![PathBuf::from("/w/core"), PathBuf::from("/w/host")],
        allowed_files: vec![PathBuf::from(ALLOCATOR)],
    }
}

#[test]
fn label_names_the_unsafe_gate() {
    // Arrange
    let sources = InMemorySourceFiles::new();

    // Act
    let label = UnsafeGate::new(&sources, rule()).label();

    // Assert
    assert_eq!(label, "Unsafe confined");
}

#[test]
fn run_of_a_core_using_unsafe_fails_naming_the_file() {
    // Arrange
    let sources = InMemorySourceFiles::new()
        .with_file("/w/core/src/lib.rs", UNSAFE)
        .with_file(ALLOCATOR, UNSAFE);

    // Act
    let outcome = UnsafeGate::new(&sources, rule()).run();

    // Assert
    assert_eq!(
        outcome,
        Err(GateFailure::Breached {
            findings: vec![String::from(
                "/w/core/src/lib.rs uses an unsafe impl, which only /w/host/src/stingy.rs may"
            )]
        })
    );
}

#[test]
fn run_of_a_directory_it_cannot_list_fails_naming_it() {
    // Arrange
    let sources = InMemorySourceFiles::new().with_file("/w/core/src/lib.rs", SAFE);

    // Act
    let outcome = UnsafeGate::new(&sources, rule()).run();

    // Assert
    assert_eq!(
        outcome,
        Err(GateFailure::SourceUnreadable(SourceReadFailure {
            path: PathBuf::from("/w/host"),
            reason: String::from("not held"),
        }))
    );
}

#[test]
fn run_of_a_file_that_does_not_parse_fails_naming_it() {
    // Arrange
    let sources = InMemorySourceFiles::new()
        .with_file("/w/core/src/lib.rs", "fn (")
        .with_file("/w/host/src/lib.rs", SAFE);

    // Act
    let outcome = UnsafeGate::new(&sources, rule()).run();

    // Assert
    assert!(matches!(
        outcome,
        Err(GateFailure::Unparsable { file, .. }) if file == "/w/core/src/lib.rs"
    ));
}

#[test]
fn run_of_two_files_using_unsafe_names_each_on_a_line() {
    // Arrange
    let sources = InMemorySourceFiles::new()
        .with_file("/w/core/src/lib.rs", UNSAFE)
        .with_file("/w/host/src/lib.rs", "unsafe fn f() {}");

    // Act
    let outcome = UnsafeGate::new(&sources, rule()).run();

    // Assert
    assert_eq!(
        outcome,
        Err(GateFailure::Breached {
            findings: vec![
                String::from(
                    "/w/core/src/lib.rs uses an unsafe impl, which only /w/host/src/stingy.rs may"
                ),
                String::from(
                    "/w/host/src/lib.rs uses an unsafe fn, which only /w/host/src/stingy.rs may"
                ),
            ]
        })
    );
}

#[test]
fn run_with_no_directory_to_scan_fails() {
    // Arrange
    let sources = InMemorySourceFiles::new();
    let nothing = UnsafeRule {
        source_dirs: vec![],
        allowed_files: vec![PathBuf::from(ALLOCATOR)],
    };

    // Act
    let outcome = UnsafeGate::new(&sources, nothing).run();

    // Assert
    assert_eq!(
        outcome,
        Err(GateFailure::Own {
            reason: String::from("no source file was scanned for unsafe")
        })
    );
}

#[test]
fn run_with_unsafe_only_in_the_allowed_file_passes() {
    // Arrange
    let sources = InMemorySourceFiles::new()
        .with_file("/w/core/src/lib.rs", SAFE)
        .with_file(ALLOCATOR, UNSAFE);

    // Act
    let outcome = UnsafeGate::new(&sources, rule()).run();

    // Assert
    assert_eq!(outcome, Ok(()));
}
