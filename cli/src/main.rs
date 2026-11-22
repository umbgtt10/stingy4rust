// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::process::ExitCode;

fn main() -> ExitCode {
    eprintln!(
        "cargo-stingy4rust is in development: this release reserves the name and checks nothing."
    );
    ExitCode::FAILURE
}
