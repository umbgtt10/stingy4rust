// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// One kind of message: how many were sent, and their bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WireFigures {
    pub kind: &'static str,
    pub messages: u64,
    pub bytes: u64,
}
