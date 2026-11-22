// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// A scope's name -- `node`, or `node 3` -- built from a `&'static str` and an
// optional number, so naming one never allocates.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScopeName {
    pub name: &'static str,
    pub number: Option<u32>,
}
