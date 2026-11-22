// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::figures::heap_figures::HeapFigures;
use stingy4rust::figures::operation_figures::OperationFigures;
use stingy4rust::figures::scope_figures::ScopeFigures;
use stingy4rust::figures::scope_name::ScopeName;
use stingy4rust::figures::store_figures::StoreFigures;
use stingy4rust::figures::tally::Tally;
use stingy4rust::figures::wire_figures::WireFigures;
use stingy4rust::ledger::bookkeeping::Bookkeeping;
use stingy4rust::ledger::ledger_tables::LedgerTables;
use stingy4rust::ledger::max_depth::MAX_DEPTH;
use stingy4rust::ledger::run_status::RunStatus;

const LEDGER_BYTES: u64 = 4096;
const OTHER_THREAD: usize = 2;
const THREAD: usize = 1;

fn begun() -> LedgerTables<4, 8, 4> {
    let mut tables = LedgerTables::new();
    tables.begin(THREAD);
    tables
}

const fn named(name: &'static str) -> ScopeName {
    ScopeName { name, number: None }
}

fn scope(tally: &Tally, name: &str) -> ScopeFigures {
    *tally
        .scopes
        .iter()
        .find(|figures| figures.name.name == name)
        .expect("the scope is in the tally")
}

#[test]
fn allocated_from_another_thread_while_a_scope_is_open_reports_foreign_thread() {
    // Arrange
    let mut tables = begun();
    tables.enter(named("worker"));

    // Act
    tables.allocated(0x1000, 64, OTHER_THREAD);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(tally.status, RunStatus::ForeignThread);
    assert_eq!(scope(&tally, "worker").heap, HeapFigures::default());
}

#[test]
fn allocated_from_another_thread_with_no_scope_open_reports_foreign_thread() {
    // Arrange
    let mut tables = begun();

    // Act
    tables.allocated(0x1000, 64, OTHER_THREAD);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(tally.status, RunStatus::ForeignThread);
    assert_eq!(tally.heap, HeapFigures::default());
}

#[test]
fn allocated_of_three_blocks_in_a_scope_charges_it_192_bytes() {
    // Arrange
    let mut tables = begun();
    tables.enter(named("worker"));

    // Act
    tables.allocated(0x1000, 64, THREAD);
    tables.allocated(0x1040, 64, THREAD);
    tables.allocated(0x1080, 64, THREAD);

    // Assert
    let worker = scope(&tables.tally(LEDGER_BYTES), "worker");
    assert_eq!(
        worker.heap,
        HeapFigures {
            allocations: 3,
            bytes_allocated: 192,
            live_at_end: 192,
            ..HeapFigures::default()
        }
    );
    assert_eq!(worker.peak, 192);
}

#[test]
fn allocated_outside_any_scope_charges_the_outside_scopes_slot() {
    // Arrange
    let mut tables = begun();

    // Act
    tables.allocated(0x1000, 32, THREAD);

    // Assert
    let outside = scope(&tables.tally(LEDGER_BYTES), "(outside scopes)");
    assert_eq!(outside.heap.allocations, 1);
    assert_eq!(outside.heap.bytes_allocated, 32);
}

#[test]
fn allocated_past_the_block_capacity_reports_blocks_exhausted() {
    // Arrange
    let mut tables = begun();

    // Act
    for block in 0..9 {
        tables.allocated(0x1000 + block * 0x10, 8, THREAD);
    }

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(tally.status, RunStatus::BlocksExhausted);
    assert_eq!(scope(&tally, "(outside scopes)").heap.allocations, 8);
}

#[test]
fn at_rest_keeps_the_last_value() {
    // Arrange
    let mut tables = begun();

    // Act
    tables.at_rest(named("journal"), 300);
    tables.at_rest(named("journal"), 150);

    // Assert
    assert_eq!(tables.tally(LEDGER_BYTES).storage[0].bytes_at_rest, 150);
}

#[test]
fn begin_forgets_the_run_before() {
    // Arrange
    let mut tables = begun();
    tables.enter(named("worker"));
    tables.allocated(0x1000, 64, THREAD);
    tables.freed(0x2000, 8, THREAD);
    tables.enter(named("a.b"));

    // Act
    tables.begin(THREAD);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(tally.status, RunStatus::Measured);
    assert_eq!(tally.heap, HeapFigures::default());
    assert_eq!(tally.peak, 0);
    assert_eq!(tally.scopes.len(), 1);
    assert_eq!(tally.bookkeeping.untracked_frees, 0);
    assert_eq!(tally.bookkeeping.tracked_blocks_peak, 0);
}

#[test]
fn begin_forgets_the_wire_stores_and_operations_of_the_run_before() {
    // Arrange
    let mut tables = begun();
    tables.sent("ping", 10);
    tables.written(named("journal"), 50);
    tables.begin_operation("exchange");

    // Act
    tables.begin(THREAD);

    // Assert
    tables.sent("pong", 100);
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(tally.storage, Vec::new());
    assert_eq!(tally.operations, Vec::new());
    assert_eq!(
        tally.wire,
        vec![WireFigures {
            kind: "pong",
            messages: 1,
            bytes: 100,
        }]
    );
}

#[test]
fn begin_operation_while_another_is_open_charges_the_wire_to_the_newer() {
    // Arrange
    let mut tables = begun();
    tables.begin_operation("outer");

    // Act
    tables.begin_operation("inner");
    tables.sent("ping", 10);

    // Assert
    let operations = tables.tally(LEDGER_BYTES).operations;
    assert_eq!(operations[0].messages, 0);
    assert_eq!(operations[1].messages, 1);
}

#[test]
fn bookkeeping_reports_the_capacities_and_the_most_blocks_tracked() {
    // Arrange
    let mut tables = begun();
    tables.allocated(0x1000, 8, THREAD);
    tables.allocated(0x1010, 8, THREAD);
    tables.freed(0x1000, 8, THREAD);
    tables.freed(0x3000, 8, THREAD);

    // Act
    let bookkeeping = tables.bookkeeping(LEDGER_BYTES);

    // Assert
    assert_eq!(
        bookkeeping,
        Bookkeeping {
            block_capacity: 8,
            tracked_blocks_peak: 2,
            scope_capacity: 4,
            name_capacity: 4,
            untracked_frees: 1,
            ledger_bytes: LEDGER_BYTES,
        }
    );
}

#[test]
fn end_operation_counts_one_and_stops_charging_it() {
    // Arrange
    let mut tables = begun();
    tables.begin_operation("exchange");
    tables.sent("ping", 10);

    // Act
    tables.end_operation("exchange");

    // Assert
    tables.sent("pong", 100);
    assert_eq!(
        tables.tally(LEDGER_BYTES).operations,
        vec![OperationFigures {
            name: "exchange",
            count: 1,
            messages: 1,
            bytes: 10,
        }]
    );
}

#[test]
fn enter_of_a_name_entered_before_counts_one_more_entry() {
    // Arrange
    let mut tables = begun();
    let depth = tables.enter(named("worker"));
    tables.leave(depth);

    // Act
    let again = tables.enter(named("worker"));

    // Assert
    tables.leave(again);
    assert_eq!(scope(&tables.tally(LEDGER_BYTES), "worker").entries, 2);
}

#[test]
fn enter_of_a_name_with_a_dot_reports_name_with_dot() {
    // Arrange
    let mut tables = begun();

    // Act
    let depth = tables.enter(named("leader.step"));

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(depth, 0);
    assert_eq!(tally.status, RunStatus::NameWithDot);
    assert_eq!(tally.scopes.len(), 1);
}

#[test]
fn enter_past_the_depth_reports_depth_exhausted() {
    // Arrange
    let mut tables = begun();
    for _ in 0..MAX_DEPTH {
        tables.enter(named("worker"));
    }

    // Act
    let depth = tables.enter(named("worker"));

    // Assert
    assert_eq!(depth, 0);
    assert_eq!(tables.tally(LEDGER_BYTES).status, RunStatus::DepthExhausted);
}

#[test]
fn enter_past_the_scope_capacity_reports_scopes_exhausted() {
    // Arrange
    let mut tables = begun();
    for name in ["a", "b", "c"] {
        let depth = tables.enter(named(name));
        tables.leave(depth);
    }

    // Act
    let depth = tables.enter(named("d"));

    // Assert
    assert_eq!(depth, 0);
    assert_eq!(
        tables.tally(LEDGER_BYTES).status,
        RunStatus::ScopesExhausted
    );
}

#[test]
fn enter_returns_the_depth_of_the_frame_it_opens() {
    // Arrange
    let mut tables = begun();

    // Act
    let outer = tables.enter(named("outer"));
    let inner = tables.enter(named("inner"));

    // Assert
    assert_eq!((outer, inner), (1, 2));
}

#[test]
fn freed_from_another_thread_reports_foreign_thread_and_frees_nothing() {
    // Arrange
    let mut tables = begun();
    tables.allocated(0x1000, 64, THREAD);

    // Act
    tables.freed(0x1000, 64, OTHER_THREAD);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(tally.status, RunStatus::ForeignThread);
    assert_eq!(tally.heap.live_at_end, 64);
}

#[test]
fn freed_of_a_block_allocated_before_begin_counts_an_untracked_free() {
    // Arrange
    let mut tables = begun();

    // Act
    tables.freed(0x1000, 64, THREAD);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(tally.bookkeeping.untracked_frees, 1);
    assert_eq!(tally.heap, HeapFigures::default());
    assert_eq!(tally.scopes[0].heap, HeapFigures::default());
}

#[test]
fn freed_of_a_block_in_another_scope_credits_the_scope_that_allocated_it() {
    // Arrange
    let mut tables = begun();
    let producer = tables.enter(named("producer"));
    tables.allocated(0x1000, 100, THREAD);
    tables.leave(producer);
    tables.enter(named("consumer"));

    // Act
    tables.freed(0x1000, 100, THREAD);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(
        scope(&tally, "producer").heap,
        HeapFigures {
            allocations: 1,
            bytes_allocated: 100,
            bytes_freed: 100,
            ..HeapFigures::default()
        }
    );
    assert_eq!(scope(&tally, "consumer").heap, HeapFigures::default());
}

#[test]
fn leave_of_no_depth_changes_nothing() {
    // Arrange
    let mut tables = begun();
    tables.enter(named("worker"));

    // Act
    tables.leave(0);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(tally.status, RunStatus::Measured);
    tables.allocated(0x1000, 8, THREAD);
    assert_eq!(
        scope(&tables.tally(LEDGER_BYTES), "worker")
            .heap
            .allocations,
        1
    );
}

#[test]
fn leave_out_of_order_reports_scope_left_out_of_order() {
    // Arrange
    let mut tables = begun();
    let outer = tables.enter(named("outer"));
    tables.enter(named("inner"));

    // Act
    tables.leave(outer);

    // Assert
    assert_eq!(
        tables.tally(LEDGER_BYTES).status,
        RunStatus::ScopeLeftOutOfOrder
    );
}

#[test]
fn on_disk_reports_the_store_s_files() {
    // Arrange
    let mut tables = begun();

    // Act
    tables.on_disk(named("journal"), 4096);

    // Assert
    assert_eq!(
        tables.tally(LEDGER_BYTES).storage[0].bytes_on_disk,
        Some(4096)
    );
}

#[test]
fn reallocated_counts_a_reallocation_freeing_the_old_size_and_charging_the_new() {
    // Arrange
    let mut tables = begun();
    tables.enter(named("grower"));
    tables.allocated(0x1000, 8, THREAD);

    // Act
    tables.reallocated(0x1000, 8, 0x2000, 16, THREAD);

    // Assert
    assert_eq!(
        scope(&tables.tally(LEDGER_BYTES), "grower").heap,
        HeapFigures {
            allocations: 1,
            reallocations: 1,
            bytes_allocated: 24,
            bytes_freed: 8,
            live_at_end: 16,
        }
    );
}

#[test]
fn reallocated_from_another_thread_reports_foreign_thread_and_moves_nothing() {
    // Arrange
    let mut tables = begun();
    tables.allocated(0x1000, 8, THREAD);

    // Act
    tables.reallocated(0x1000, 8, 0x2000, 16, OTHER_THREAD);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(tally.status, RunStatus::ForeignThread);
    assert_eq!(tally.heap.live_at_end, 8);
}

#[test]
fn reallocated_in_another_scope_charges_the_new_block_to_the_scope_open() {
    // Arrange
    let mut tables = begun();
    let first = tables.enter(named("first"));
    tables.allocated(0x1000, 8, THREAD);
    tables.leave(first);
    tables.enter(named("second"));

    // Act
    tables.reallocated(0x1000, 8, 0x2000, 16, THREAD);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(
        scope(&tally, "first").heap,
        HeapFigures {
            allocations: 1,
            bytes_allocated: 8,
            bytes_freed: 8,
            ..HeapFigures::default()
        }
    );
    assert_eq!(
        scope(&tally, "second").heap,
        HeapFigures {
            reallocations: 1,
            bytes_allocated: 16,
            live_at_end: 16,
            ..HeapFigures::default()
        }
    );
}

#[test]
fn reallocated_of_a_block_the_table_never_held_counts_an_untracked_free_and_an_allocation() {
    // Arrange
    let mut tables = begun();

    // Act
    tables.reallocated(0x1000, 8, 0x2000, 16, THREAD);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(tally.bookkeeping.untracked_frees, 1);
    assert_eq!(
        tally.heap,
        HeapFigures {
            allocations: 1,
            bytes_allocated: 16,
            live_at_end: 16,
            ..HeapFigures::default()
        }
    );
}

#[test]
fn sent_of_a_kind_with_a_dot_reports_name_with_dot() {
    // Arrange
    let mut tables = begun();

    // Act
    tables.sent("log.append", 10);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(tally.status, RunStatus::NameWithDot);
    assert_eq!(tally.wire, Vec::new());
}

#[test]
fn sent_past_the_name_capacity_reports_names_exhausted() {
    // Arrange
    let mut tables = begun();
    for kind in ["a", "b", "c", "d"] {
        tables.sent(kind, 1);
    }

    // Act
    tables.sent("e", 1);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(tally.status, RunStatus::NamesExhausted);
    assert_eq!(tally.wire.len(), 4);
}

#[test]
fn sent_while_an_operation_is_open_charges_it() {
    // Arrange
    let mut tables = begun();
    tables.begin_operation("exchange");

    // Act
    tables.sent("ping", 10);
    tables.sent("pong", 100);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(
        tally.operations,
        vec![OperationFigures {
            name: "exchange",
            count: 0,
            messages: 2,
            bytes: 110,
        }]
    );
    assert_eq!(tally.wire[1].bytes, 100);
}

#[test]
fn sent_with_no_operation_open_charges_the_kind_alone() {
    // Arrange
    let mut tables = begun();

    // Act
    tables.sent("ping", 10);
    tables.sent("ping", 10);

    // Assert
    let tally = tables.tally(LEDGER_BYTES);
    assert_eq!(
        tally.wire,
        vec![WireFigures {
            kind: "ping",
            messages: 2,
            bytes: 20,
        }]
    );
    assert_eq!(tally.operations, Vec::new());
}

#[test]
fn tally_keeps_the_first_failure() {
    // Arrange
    let mut tables = begun();
    tables.enter(named("a.b"));

    // Act
    tables.allocated(0x1000, 8, OTHER_THREAD);

    // Assert
    assert_eq!(tables.tally(LEDGER_BYTES).status, RunStatus::NameWithDot);
}

#[test]
fn tally_lists_the_outside_scopes_slot_first_then_scopes_as_first_entered() {
    // Arrange
    let mut tables = begun();
    for name in ["second", "first", "second"] {
        let depth = tables.enter(named(name));
        tables.leave(depth);
    }

    // Act
    let tally = tables.tally(LEDGER_BYTES);

    // Assert
    let names: Vec<&str> = tally
        .scopes
        .iter()
        .map(|figures| figures.name.name)
        .collect();
    assert_eq!(names, vec!["(outside scopes)", "second", "first"]);
}

#[test]
fn tally_of_a_scope_entry_that_rose_and_fell_reports_its_largest_rise() {
    // Arrange
    let mut tables = begun();
    let depth = tables.enter(named("step"));
    tables.allocated(0x1000, 100, THREAD);
    tables.freed(0x1000, 100, THREAD);
    tables.allocated(0x1000, 30, THREAD);
    tables.freed(0x1000, 30, THREAD);
    tables.leave(depth);

    // Act
    let tally = tables.tally(LEDGER_BYTES);

    // Assert
    let step = scope(&tally, "step");
    assert_eq!(step.largest_entry_rise, 100);
    assert_eq!(step.peak, 100);
    assert_eq!(step.heap.live_at_end, 0);
}

#[test]
fn tally_of_an_entry_begun_with_bytes_live_reports_the_rise_above_them() {
    // Arrange
    let mut tables = begun();
    let first = tables.enter(named("step"));
    tables.allocated(0x1000, 50, THREAD);
    tables.leave(first);
    let second = tables.enter(named("step"));
    tables.allocated(0x2000, 20, THREAD);
    tables.leave(second);

    // Act
    let tally = tables.tally(LEDGER_BYTES);

    // Assert
    let step = scope(&tally, "step");
    assert_eq!(step.largest_entry_rise, 50);
    assert_eq!(step.peak, 70);
}

#[test]
fn tally_reports_the_run_peak_across_scopes() {
    // Arrange
    let mut tables = begun();
    let first = tables.enter(named("first"));
    tables.allocated(0x1000, 10, THREAD);
    tables.leave(first);
    tables.enter(named("second"));
    tables.allocated(0x2000, 20, THREAD);
    tables.freed(0x2000, 20, THREAD);

    // Act
    let tally = tables.tally(LEDGER_BYTES);

    // Assert
    assert_eq!(tally.peak, 30);
}

#[test]
fn tally_sums_the_run_s_heap_over_every_scope() {
    // Arrange
    let mut tables = begun();
    tables.allocated(0x1000, 10, THREAD);
    let depth = tables.enter(named("worker"));
    tables.allocated(0x2000, 20, THREAD);
    tables.reallocated(0x2000, 20, 0x3000, 40, THREAD);
    tables.leave(depth);
    tables.freed(0x1000, 10, THREAD);

    // Act
    let tally = tables.tally(LEDGER_BYTES);

    // Assert
    assert_eq!(
        tally.heap,
        HeapFigures {
            allocations: 2,
            reallocations: 1,
            bytes_allocated: 70,
            bytes_freed: 30,
            live_at_end: 40,
        }
    );
    assert_eq!(tally.bookkeeping.ledger_bytes, LEDGER_BYTES);
}

#[test]
fn transaction_counts_one() {
    // Arrange
    let mut tables = begun();

    // Act
    tables.transaction(named("journal"));

    // Assert
    assert_eq!(tables.tally(LEDGER_BYTES).storage[0].transactions, 1);
}

#[test]
fn written_counts_bytes_and_writes_of_the_store() {
    // Arrange
    let mut tables = begun();

    // Act
    tables.written(named("journal"), 50);
    tables.written(named("journal"), 50);

    // Assert
    assert_eq!(
        tables.tally(LEDGER_BYTES).storage,
        vec![StoreFigures {
            store: named("journal"),
            bytes_written: 100,
            writes: 2,
            transactions: 0,
            bytes_at_rest: 0,
            bytes_on_disk: None,
        }]
    );
}
