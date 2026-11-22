// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// What one invocation ends with: what it prints to standard output and to
// standard error, and its exit code -- 0 within budget, 1 over it, 2 unable
// to measure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: u8,
}
