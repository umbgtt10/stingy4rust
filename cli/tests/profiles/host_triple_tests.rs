// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fake_command_runner::FakeCommandRunner;
use crate::support::sample::RUSTC;
use crate::support::sample::TARGET;
use stingy4rust_cli::profiles::host_triple::HostTriple;
use stingy4rust_cli::unable::Unable;

#[test]
fn read_of_rustc_s_version_is_its_host_line() {
    // Arrange
    let runner = FakeCommandRunner::new().answering("rustc -vV", Some(0), RUSTC);

    // Act
    let triple = HostTriple::new(&runner).read();

    // Assert
    assert_eq!(triple, Ok(String::from(TARGET)));
}

#[test]
fn read_when_rustc_fails_is_unable() {
    // Arrange
    let runner = FakeCommandRunner::new().answering("rustc -vV", Some(1), RUSTC);

    // Act
    let triple = HostTriple::new(&runner).read();

    // Assert
    assert_eq!(
        triple,
        Err(Unable::Failed {
            command: String::from("rustc -vV"),
            code: Some(1),
        })
    );
}

#[test]
fn read_without_a_host_line_is_unable() {
    // Arrange
    let runner = FakeCommandRunner::new().answering("rustc -vV", Some(0), "rustc 1.99.1\n");

    // Act
    let triple = HostTriple::new(&runner).read();

    // Assert
    assert_eq!(triple, Err(Unable::NoHostTriple));
}
