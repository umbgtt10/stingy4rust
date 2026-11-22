// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// One live block: where it is, its size, and the slot of the scope charged
// with it -- which a free credits, whoever frees it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockEntry {
    pub address: usize,
    pub size: u64,
    pub scope: usize,
}
