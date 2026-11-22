// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// The reports of two runs of the same profiles, in the order run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwoReports {
    pub first: String,
    pub second: String,
}
