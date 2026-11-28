// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

//! The measuring behind `cargo-stingy4rust`: a ledger that counts a run's heap
//! by scope, and its wire, stores and operations, never allocating while it
//! counts; the runs and profiles that reach it; the figures it hands back, the
//! report they are written as, and the budgets they are held to.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod budget;
pub mod figures;
pub mod ledger;
pub mod report;
pub mod run;
