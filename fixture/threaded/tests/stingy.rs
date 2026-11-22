// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::alloc::System;
use std::hint::black_box;
use std::process::ExitCode;
use std::thread;
use stingy4rust::run::profile::Profile;
use stingy4rust::run::run::Run;
use stingy4rust_host::file_report_sink::FileReportSink;
use stingy4rust_host::host_stingy::HostStingy;
use stingy4rust_host::process_environment::ProcessEnvironment;
use stingy4rust_host::profile_main::ProfileMain;
use stingy4rust_host::stingy::Stingy;

#[global_allocator]
static STINGY: HostStingy = Stingy::new(System);

static PROFILES: [Profile; 1] = [Profile {
    name: "second-thread",
    seed: 2026,
    body: second_thread,
}];

fn main() -> ExitCode {
    ProfileMain::new(&STINGY, &PROFILES, &ProcessEnvironment, &FileReportSink).run()
}

// A second thread allocates while scope `spawner` is open.
fn second_thread(run: &Run) {
    let _spawner = run.scope("spawner");
    thread::spawn(|| drop(black_box(Box::new([0_u8; 16]))))
        .join()
        .expect("the thread runs");
}
