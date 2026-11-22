// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

use crate::budget::figure_verdict::FigureVerdict;

// One profile compared: each of its figures' verdicts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileCheck {
    pub name: String,
    pub figures: Vec<FigureVerdict>,
}
