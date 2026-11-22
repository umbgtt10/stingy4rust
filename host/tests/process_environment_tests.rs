// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust_host::environment::Environment;
use stingy4rust_host::process_environment::ProcessEnvironment;

#[test]
fn variable_cargo_sets_for_a_test_is_its_value() {
    // Arrange
    let environment = ProcessEnvironment;

    // Act
    let name = environment.variable("CARGO_PKG_NAME");

    // Assert
    assert_eq!(name.as_deref(), Some("stingy4rust-host"));
}

#[test]
fn variable_nobody_set_is_none() {
    // Arrange
    let environment = ProcessEnvironment;

    // Act
    let value = environment.variable("STINGY4RUST_NOBODY_SETS_THIS");

    // Assert
    assert_eq!(value, None);
    assert!(environment.variable("CARGO_PKG_NAME").is_some());
}
