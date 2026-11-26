// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::figures::heap_figures::HeapFigures;
use stingy4rust::figures::operation_figures::OperationFigures;
use stingy4rust::figures::scope_name::ScopeName;
use stingy4rust::ledger::ledger::Ledger;
use stingy4rust::ledger::run_status::RunStatus;
use stingy4rust::run::recorder::Recorder;

static SHARED: Ledger<4, 8, 4> = Ledger::new();
const THREAD: usize = 1;

const fn worker() -> ScopeName {
    ScopeName {
        name: "worker",
        number: None,
    }
}

#[test]
fn allocated_after_end_is_not_counted() {
    // Arrange
    let ledger = Ledger::<4, 8, 4>::new();
    ledger.begin(THREAD);
    ledger.allocated(0x1000, 64, THREAD);
    let _ = ledger.end();

    // Act
    ledger.allocated(0x2000, 64, THREAD);

    // Assert
    assert_eq!(ledger.end().heap.allocations, 1);
}

#[test]
fn allocated_before_begin_is_not_counted() {
    // Arrange
    let ledger = Ledger::<4, 8, 4>::new();

    // Act
    ledger.allocated(0x1000, 64, THREAD);

    // Assert
    let bookkeeping = ledger.bookkeeping();
    assert_eq!(bookkeeping.tracked_blocks_peak, 0);
    assert_eq!(bookkeeping.block_capacity, 8);
}

#[test]
fn bookkeeping_reports_the_ledger_s_own_size() {
    // Arrange
    let ledger = Ledger::<4, 8, 4>::new();

    // Act
    let bookkeeping = ledger.bookkeeping();

    // Assert
    assert_eq!(
        bookkeeping.ledger_bytes,
        size_of::<Ledger<4, 8, 4>>() as u64
    );
    assert_eq!(bookkeeping.block_capacity, 8);
}

#[test]
fn end_hands_back_what_was_counted_between_begin_and_end() {
    // Arrange
    let ledger = Ledger::<4, 8, 4>::new();
    ledger.begin(THREAD);
    let depth = ledger.enter(worker());
    ledger.allocated(0x1000, 64, THREAD);
    ledger.reallocated(0x1000, 64, 0x2000, 128, THREAD);
    ledger.freed(0x2000, 128, THREAD);
    ledger.leave(depth);

    // Act
    let tally = ledger.end();

    // Assert
    assert_eq!(tally.status, RunStatus::Measured);
    assert_eq!(
        tally.heap,
        HeapFigures {
            allocations: 1,
            reallocations: 1,
            bytes_allocated: 192,
            bytes_freed: 192,
            live_at_end: 0,
        }
    );
    assert_eq!(tally.scopes[1].name, worker());
    assert_eq!(tally.scopes[1].entries, 1);
    assert_eq!(
        tally.bookkeeping.ledger_bytes,
        size_of::<Ledger<4, 8, 4>>() as u64
    );
}

#[test]
fn enter_before_begin_opens_no_scope() {
    // Arrange
    let ledger = Ledger::<4, 8, 4>::new();

    // Act
    let depth = ledger.enter(worker());

    // Assert
    assert_eq!(depth, 0);
    ledger.begin(THREAD);
    assert_eq!(ledger.end().scopes.len(), 1);
}

#[test]
fn freed_after_end_is_not_counted() {
    // Arrange
    let ledger = Ledger::<4, 8, 4>::new();
    ledger.begin(THREAD);
    ledger.allocated(0x1000, 64, THREAD);
    let _ = ledger.end();

    // Act
    ledger.freed(0x1000, 64, THREAD);

    // Assert
    let heap = ledger.end().heap;
    assert_eq!(heap.bytes_freed, 0);
    assert_eq!(heap.live_at_end, 64);
}

#[test]
fn leave_after_end_changes_nothing() {
    // Arrange
    let ledger = Ledger::<4, 8, 4>::new();
    ledger.begin(THREAD);
    let _ = ledger.enter(worker());
    let _ = ledger.enter(worker());
    let _ = ledger.end();

    // Act
    ledger.leave(1);

    // Assert
    let tally = ledger.end();
    assert_eq!(tally.status, RunStatus::Measured);
    assert_eq!(tally.scopes[1].entries, 2);
}

#[test]
fn leave_during_a_run_records_the_rise_of_the_entry_it_closes() {
    // Arrange
    let ledger = Ledger::<4, 8, 4>::new();
    ledger.begin(THREAD);
    let depth = ledger.enter(worker());
    ledger.allocated(0x1000, 64, THREAD);

    // Act
    ledger.leave(depth);

    // Assert
    let tally = ledger.end();
    assert_eq!(tally.status, RunStatus::Measured);
    assert_eq!(tally.scopes[1].largest_entry_rise, 64);
}

#[test]
fn new_builds_a_ledger_a_static_can_hold() {
    // Arrange
    SHARED.begin(THREAD);

    // Act
    SHARED.allocated(0x1000, 8, THREAD);

    // Assert
    assert_eq!(SHARED.end().heap.allocations, 1);
}

#[test]
fn reallocated_after_end_is_not_counted() {
    // Arrange
    let ledger = Ledger::<4, 8, 4>::new();
    ledger.begin(THREAD);
    ledger.allocated(0x1000, 8, THREAD);
    let _ = ledger.end();

    // Act
    ledger.reallocated(0x1000, 8, 0x2000, 16, THREAD);

    // Assert
    let heap = ledger.end().heap;
    assert_eq!(heap.reallocations, 0);
    assert_eq!(heap.allocations, 1);
}

#[test]
fn sent_before_begin_is_not_counted() {
    // Arrange
    let ledger = Ledger::<4, 8, 4>::new();
    ledger.sent("ping", 10);
    ledger.begin(THREAD);

    // Act
    ledger.sent("pong", 100);

    // Assert
    let wire = ledger.end().wire;
    assert_eq!(wire.len(), 1);
    assert_eq!(wire[0].kind, "pong");
}

#[test]
fn sent_while_an_operation_is_open_charges_it() {
    // Arrange
    let ledger = Ledger::<4, 8, 4>::new();
    ledger.begin(THREAD);
    ledger.begin_operation("exchange");

    // Act
    ledger.sent("ping", 10);

    // Assert
    assert_eq!(
        ledger.end().operations,
        vec![OperationFigures {
            name: "exchange",
            count: 0,
            messages: 1,
            bytes: 10,
        }]
    );
}

#[test]
fn written_during_a_run_reaches_the_tally_with_the_operations() {
    // Arrange
    let ledger = Ledger::<4, 8, 4>::new();
    ledger.begin(THREAD);
    ledger.begin_operation("flush");

    // Act
    ledger.written(worker(), 50);
    ledger.transaction(worker());
    ledger.at_rest(worker(), 50);
    ledger.on_disk(worker(), 4096);
    ledger.end_operation("flush");

    // Assert
    let tally = ledger.end();
    assert_eq!(tally.storage[0].bytes_written, 50);
    assert_eq!(tally.storage[0].transactions, 1);
    assert_eq!(tally.storage[0].bytes_at_rest, 50);
    assert_eq!(tally.storage[0].bytes_on_disk, Some(4096));
    assert_eq!(tally.operations[0].count, 1);
}
