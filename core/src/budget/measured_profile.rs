// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::vec::Vec;

use crate::figures::figure::Figure;

// One profile's figures as a report gives them back: its name and every
// figure, in the order written.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeasuredProfile {
    pub name: String,
    pub figures: Vec<Figure>,
}
