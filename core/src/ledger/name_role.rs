// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// What a name stands for in a run: a kind of message, a store, or an
// operation. One name may stand for more than one, apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameRole {
    Kind,
    Store,
    Operation,
}
