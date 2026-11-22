// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::alloc::System;
use std::array::from_fn;
use std::hint::black_box;
use std::process::ExitCode;
use stingy4rust::run::profile::Profile;
use stingy4rust::run::run::Run;
use stingy4rust_host::file_report_sink::FileReportSink;
use stingy4rust_host::process_environment::ProcessEnvironment;
use stingy4rust_host::profile_main::ProfileMain;
use stingy4rust_host::stingy::Stingy;

#[global_allocator]
static STINGY: Stingy<System, 4, 8, 4> = Stingy::new(System);

static PROFILES: [Profile; 1] = [Profile {
    name: "nine-blocks",
    seed: 2026,
    body: nine_blocks,
}];

fn main() -> ExitCode {
    ProfileMain::new(&STINGY, &PROFILES, &ProcessEnvironment, &FileReportSink).run()
}

// Nine blocks alive at once, one more than the ledger holds.
fn nine_blocks(run: &Run) {
    let _hoarder = run.scope("hoarder");
    let blocks: [Box<u64>; 9] = from_fn(|index| black_box(Box::new(index as u64)));
    drop(black_box(blocks));
}
