// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// One open entry of a scope: its slot, the bytes live in it when the entry
// began, and the most live in it since.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame {
    pub slot: usize,
    pub base: u64,
    pub high: u64,
}
