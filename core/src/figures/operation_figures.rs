// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// One kind of operation: how many there were, and the messages and bytes sent
// while one of them was open.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationFigures {
    pub name: &'static str,
    pub count: u64,
    pub messages: u64,
    pub bytes: u64,
}
