// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::alloc::System;
use stingy4rust::figures::scope_name::ScopeName;
use stingy4rust::ledger::run_status::RunStatus;
use stingy4rust::run::measuring::Measuring;
use stingy4rust::run::recorder::Recorder;
use stingy4rust_host::stingy::Stingy;

const WORKER: ScopeName = ScopeName {
    name: "worker",
    number: None,
};

#[test]
fn end_hands_back_the_scopes_entered_since_begin() {
    // Arrange
    static STINGY: Stingy<System, 4, 8, 4> = Stingy::new(System);
    STINGY.begin();
    let depth = STINGY.enter(WORKER);
    STINGY.leave(depth);

    // Act
    let tally = STINGY.end();

    // Assert
    assert_eq!(tally.status, RunStatus::Measured);
    assert_eq!(tally.scopes[1].name, WORKER);
    assert_eq!(tally.scopes[1].entries, 1);
}

#[test]
fn enter_out_of_a_run_opens_no_scope() {
    // Arrange
    static STINGY: Stingy<System, 4, 8, 4> = Stingy::new(System);

    // Act
    let depth = STINGY.enter(WORKER);

    // Assert
    assert_eq!(depth, 0);
    STINGY.begin();
    assert_eq!(STINGY.enter(WORKER), 1);
    let tally = STINGY.end();
    assert_eq!(tally.scopes.len(), 2);
    assert_eq!(tally.scopes[1].entries, 1);
}

#[test]
fn sent_and_the_store_s_calls_reach_the_tally() {
    // Arrange
    static STINGY: Stingy<System, 4, 8, 4> = Stingy::new(System);
    STINGY.begin();
    STINGY.begin_operation("exchange");

    // Act
    STINGY.sent("ping", 10);
    STINGY.end_operation("exchange");
    STINGY.written(WORKER, 50);
    STINGY.transaction(WORKER);
    STINGY.at_rest(WORKER, 50);
    STINGY.on_disk(WORKER, 4096);

    // Assert
    let tally = STINGY.end();
    assert_eq!(tally.wire[0].bytes, 10);
    assert_eq!(tally.operations[0].count, 1);
    assert_eq!(tally.operations[0].bytes, 10);
    assert_eq!(tally.storage[0].bytes_written, 50);
    assert_eq!(tally.storage[0].transactions, 1);
    assert_eq!(tally.storage[0].bytes_at_rest, 50);
    assert_eq!(tally.storage[0].bytes_on_disk, Some(4096));
}
