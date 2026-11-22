// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

//! The host's side of `cargo-stingy4rust`: the counting allocator, the thread
//! mark, and `ProfileMain`, the profile target's `main`.

#![deny(unsafe_code)]

pub mod environment;
pub mod file_report_sink;
pub mod host_stingy;
pub mod process_environment;
pub mod profile_main;
pub mod report_sink;
pub mod sink_failure;
pub mod stingy;
pub mod thread_mark;
