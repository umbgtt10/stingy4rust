// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// Whether a run could be measured, and if not, the first reason it could not:
// a table that filled, a scope nested too deep, another thread allocating, a
// guard dropped out of order, or a name that would break a figure's path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStatus {
    Measured,
    ScopesExhausted,
    BlocksExhausted,
    NamesExhausted,
    DepthExhausted,
    ForeignThread,
    ScopeLeftOutOfOrder,
    NameWithDot,
}
