// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::subcommand_run::SubcommandRun;

#[test]
fn of_a_request_for_its_version_prints_it() {
    // Arrange & Act
    let run = SubcommandRun::of(&["--version"]);

    // Assert
    assert_eq!(run.exit_code(), Some(0), "{}", run.stderr());
    assert!(
        run.stdout().starts_with("cargo-stingy4rust "),
        "{}",
        run.stdout()
    );
}
