// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::alloc::System;
use std::hint::black_box;
use std::process::ExitCode;
use stingy4rust::run::profile::Profile;
use stingy4rust::run::run::Run;
use stingy4rust_host::file_report_sink::FileReportSink;
use stingy4rust_host::host_stingy::HostStingy;
use stingy4rust_host::process_environment::ProcessEnvironment;
use stingy4rust_host::profile_main::ProfileMain;
use stingy4rust_host::stingy::Stingy;

const SEED: u64 = 2026;

#[global_allocator]
static STINGY: HostStingy = Stingy::new(System);

static PROFILES: [Profile; 5] = [
    Profile {
        name: "three-blocks",
        seed: SEED,
        body: three_blocks,
    },
    Profile {
        name: "freed-elsewhere",
        seed: SEED,
        body: freed_elsewhere,
    },
    Profile {
        name: "nested",
        seed: SEED,
        body: nested,
    },
    Profile {
        name: "growing",
        seed: SEED,
        body: growing,
    },
    Profile {
        name: "outside",
        seed: SEED,
        body: outside,
    },
];

fn main() -> ExitCode {
    ProfileMain::new(&STINGY, &PROFILES, &ProcessEnvironment, &FileReportSink).run()
}

// Three blocks of 64 bytes in scope `worker`, dropped before the guard.
fn three_blocks(run: &Run) {
    let _worker = run.scope("worker");
    let blocks = [
        black_box(Box::new([0_u8; 64])),
        black_box(Box::new([0_u8; 64])),
        black_box(Box::new([0_u8; 64])),
    ];
    drop(black_box(blocks));
}

// A block of 100 bytes made in scope `producer` and dropped in `consumer`.
fn freed_elsewhere(run: &Run) {
    let block = {
        let _producer = run.scope("producer");
        black_box(Box::new([0_u8; 100]))
    };
    let _consumer = run.scope("consumer");
    drop(black_box(block));
}

// Scope `outer` keeps 10 bytes while scope `inner` keeps 20.
fn nested(run: &Run) {
    let _outer = run.scope("outer");
    let kept = black_box(Box::new([0_u8; 10]));
    {
        let _inner = run.scope("inner");
        let also = black_box(Box::new([0_u8; 20]));
        drop(black_box(also));
    }
    drop(black_box(kept));
}

// A Vec<u8> pushed 0 to 99 in scope `grower`: it grows 8, 16, 32, 64, 128.
fn growing(run: &Run) {
    let _grower = run.scope("grower");
    let mut bytes: Vec<u8> = Vec::new();
    for byte in 0..100_u8 {
        bytes.push(black_box(byte));
    }
    drop(black_box(bytes));
}

// A block of 32 bytes made and dropped with no scope open.
fn outside(_run: &Run) {
    drop(black_box(Box::new([0_u8; 32])));
}
