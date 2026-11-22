// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

//! The measuring behind `cargo-stingy4rust`: the tables a ledger counts a
//! run's heap in -- live blocks, open scope entries, scopes -- never
//! allocating while it counts, and the figures they hold.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod figures;
pub mod ledger;
