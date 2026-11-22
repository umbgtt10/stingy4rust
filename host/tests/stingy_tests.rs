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
