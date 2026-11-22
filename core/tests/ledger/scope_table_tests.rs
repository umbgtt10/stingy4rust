// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::figures::heap_figures::HeapFigures;
use stingy4rust::figures::scope_figures::ScopeFigures;
use stingy4rust::figures::scope_name::ScopeName;
use stingy4rust::ledger::scope_table::ScopeTable;

const OUTSIDE: ScopeName = ScopeName {
    name: "(outside scopes)",
    number: None,
};

const fn named(name: &'static str) -> ScopeName {
    ScopeName { name, number: None }
}

fn untouched(name: ScopeName) -> ScopeFigures {
    ScopeFigures {
        name,
        heap: HeapFigures::default(),
        peak: 0,
        largest_entry_rise: 0,
        entries: 0,
    }
}

fn with_worker() -> ScopeTable<4> {
    let mut table = ScopeTable::new();
    table.intern(named("worker"));
    table
}

#[test]
fn allocated_charges_the_slot_and_lifts_its_peak() {
    // Arrange
    let mut table = with_worker();

    // Act
    let first = table.allocated(1, 100);
    let second = table.allocated(1, 20);

    // Assert
    assert_eq!((first, second), (100, 120));
    assert_eq!(
        table.figures()[1],
        ScopeFigures {
            heap: HeapFigures {
                allocations: 2,
                bytes_allocated: 120,
                live_at_end: 120,
                ..HeapFigures::default()
            },
            peak: 120,
            ..untouched(named("worker"))
        }
    );
}

#[test]
fn allocated_to_a_slot_out_of_range_changes_nothing() {
    // Arrange
    let mut table = with_worker();

    // Act
    let live = table.allocated(9, 100);

    // Assert
    assert_eq!(live, 0);
    assert_eq!(
        table.figures(),
        vec![untouched(OUTSIDE), untouched(named("worker"))]
    );
}

#[test]
fn clear_keeps_only_a_fresh_outside_scopes_slot() {
    // Arrange
    let mut table = with_worker();
    table.allocated(0, 10);

    // Act
    table.clear();

    // Assert
    assert_eq!(table.figures(), vec![untouched(OUTSIDE)]);
}

#[test]
fn entered_counts_one_entry() {
    // Arrange
    let mut table = with_worker();

    // Act
    table.entered(1);
    table.entered(1);

    // Assert
    assert_eq!(table.figures()[1].entries, 2);
}

#[test]
fn figures_of_a_new_table_lists_only_the_outside_scopes_slot() {
    // Arrange
    let table = ScopeTable::<4>::new();

    // Act
    let figures = table.figures();

    // Assert
    assert_eq!(figures, vec![untouched(OUTSIDE)]);
}

#[test]
fn freed_credits_the_slot_and_keeps_its_peak() {
    // Arrange
    let mut table = with_worker();
    table.allocated(1, 100);

    // Act
    table.freed(1, 100);

    // Assert
    assert_eq!(
        table.figures()[1],
        ScopeFigures {
            heap: HeapFigures {
                allocations: 1,
                bytes_allocated: 100,
                bytes_freed: 100,
                ..HeapFigures::default()
            },
            peak: 100,
            ..untouched(named("worker"))
        }
    );
}

#[test]
fn freed_of_more_than_is_live_stops_at_zero() {
    // Arrange
    let mut table = with_worker();
    table.allocated(1, 10);

    // Act
    table.freed(1, 30);

    // Assert
    assert_eq!(table.live(1), 0);
    assert_eq!(table.figures()[1].heap.bytes_freed, 30);
}

#[test]
fn intern_of_a_name_met_before_is_its_slot() {
    // Arrange
    let mut table = with_worker();

    // Act
    let slot = table.intern(named("worker"));

    // Assert
    assert_eq!(slot, Some(1));
}

#[test]
fn intern_of_a_new_name_takes_the_next_slot() {
    // Arrange
    let mut table = with_worker();

    // Act
    let slot = table.intern(named("reader"));

    // Assert
    assert_eq!(slot, Some(2));
}

#[test]
fn intern_of_a_numbered_name_is_apart_from_the_unnumbered_one() {
    // Arrange
    let mut table = with_worker();

    // Act
    let slot = table.intern(ScopeName {
        name: "worker",
        number: Some(3),
    });

    // Assert
    assert_eq!(slot, Some(2));
}

#[test]
fn intern_of_the_outside_scopes_name_is_slot_zero() {
    // Arrange
    let mut table = ScopeTable::<4>::new();

    // Act
    let slot = table.intern(OUTSIDE);

    // Assert
    assert_eq!(slot, Some(0));
}

#[test]
fn intern_past_the_capacity_returns_none() {
    // Arrange
    let mut table = with_worker();

    // Act
    let slots = [
        table.intern(named("reader")),
        table.intern(named("writer")),
        table.intern(named("one too many")),
    ];

    // Assert
    assert_eq!(slots, [Some(2), Some(3), None]);
}

#[test]
fn live_of_a_slot_is_what_it_holds_now() {
    // Arrange
    let mut table = with_worker();
    table.allocated(1, 100);
    table.allocated(1, 20);
    table.freed(1, 100);

    // Act
    let live = table.live(1);

    // Assert
    assert_eq!(live, 20);
}

#[test]
fn reallocated_counts_a_reallocation_and_charges_the_new_size() {
    // Arrange
    let mut table = with_worker();

    // Act
    let live = table.reallocated(1, 16);

    // Assert
    assert_eq!(live, 16);
    assert_eq!(
        table.figures()[1].heap,
        HeapFigures {
            reallocations: 1,
            bytes_allocated: 16,
            live_at_end: 16,
            ..HeapFigures::default()
        }
    );
}

#[test]
fn rose_keeps_the_largest_rise() {
    // Arrange
    let mut table = with_worker();

    // Act
    table.rose(1, 50);
    table.rose(1, 30);

    // Assert
    assert_eq!(table.figures()[1].largest_entry_rise, 50);
}
