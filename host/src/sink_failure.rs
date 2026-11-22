// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt;

// A report that could not be written: where it was to go, and why not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SinkFailure {
    pub path: String,
    pub reason: String,
}

impl fmt::Display for SinkFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "could not write the report to {}: {}",
            self.path, self.reason
        )
    }
}
