// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

//! The measuring behind `cargo-stingy4rust`: a ledger that counts a run's heap
//! by scope, never allocating while it counts, and the figures it hands back.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod figures;
pub mod ledger;
pub mod run;
