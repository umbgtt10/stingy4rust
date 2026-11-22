// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust_host::sink_failure::SinkFailure;

#[test]
fn to_string_names_the_path_and_the_reason() {
    // Arrange
    let failure = SinkFailure {
        path: String::from("/reports/measured.json"),
        reason: String::from("permission denied"),
    };

    // Act
    let text = failure.to_string();

    // Assert
    assert_eq!(
        text,
        "could not write the report to /reports/measured.json: permission denied"
    );
}
