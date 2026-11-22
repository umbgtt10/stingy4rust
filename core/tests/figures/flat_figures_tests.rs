// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::figures::figure::Figure;
use stingy4rust::figures::flat_figures::FlatFigures;
use stingy4rust::figures::heap_figures::HeapFigures;
use stingy4rust::figures::scope_figures::ScopeFigures;
use stingy4rust::figures::scope_name::ScopeName;
use stingy4rust::figures::tally::Tally;
use stingy4rust::ledger::bookkeeping::Bookkeeping;
use stingy4rust::ledger::run_status::RunStatus;

fn figure(path: &str, value: u64) -> Figure {
    Figure {
        path: String::from(path),
        value,
    }
}

const fn scope(
    name: &'static str,
    number: Option<u32>,
    heap: HeapFigures,
    peak: u64,
) -> ScopeFigures {
    ScopeFigures {
        name: ScopeName { name, number },
        heap,
        peak,
        largest_entry_rise: peak,
        entries: 1,
    }
}

const fn tally(status: RunStatus, scopes: Vec<ScopeFigures>) -> Tally {
    Tally {
        status,
        heap: HeapFigures {
            allocations: 3,
            reallocations: 0,
            bytes_allocated: 192,
            bytes_freed: 192,
            live_at_end: 0,
        },
        peak: 192,
        scopes,
        wire: Vec::new(),
        storage: Vec::new(),
        operations: Vec::new(),
        bookkeeping: Bookkeeping {
            block_capacity: 8,
            tracked_blocks_peak: 3,
            scope_capacity: 4,
            name_capacity: 4,
            untracked_frees: 0,
            ledger_bytes: 4096,
        },
    }
}

#[test]
fn of_a_numbered_scope_writes_its_number_in_the_path() {
    // Arrange
    let tally = tally(
        RunStatus::Measured,
        vec![scope("node", Some(3), HeapFigures::default(), 40)],
    );

    // Act
    let figures = FlatFigures::of(&tally);

    // Assert
    assert!(figures.contains(&figure("scopes.node 3.peak", 40)));
}

#[test]
fn of_a_tally_lists_heap_peaks_then_each_scope_s_figures_in_order() {
    // Arrange
    let worker = HeapFigures {
        allocations: 3,
        reallocations: 1,
        bytes_allocated: 192,
        bytes_freed: 128,
        live_at_end: 64,
    };
    let tally = tally(
        RunStatus::Measured,
        vec![
            scope("(outside scopes)", None, HeapFigures::default(), 0),
            scope("worker", None, worker, 192),
        ],
    );

    // Act
    let figures = FlatFigures::of(&tally);

    // Assert
    assert_eq!(
        figures,
        vec![
            figure("heap.allocations", 3),
            figure("heap.reallocations", 0),
            figure("heap.bytes_allocated", 192),
            figure("heap.bytes_freed", 192),
            figure("heap.live_at_end", 0),
            figure("peaks.run", 192),
            figure("scopes.(outside scopes).allocations", 0),
            figure("scopes.(outside scopes).reallocations", 0),
            figure("scopes.(outside scopes).bytes_allocated", 0),
            figure("scopes.(outside scopes).bytes_freed", 0),
            figure("scopes.(outside scopes).live_at_end", 0),
            figure("scopes.(outside scopes).peak", 0),
            figure("scopes.(outside scopes).largest_entry_rise", 0),
            figure("scopes.(outside scopes).entries", 1),
            figure("scopes.worker.allocations", 3),
            figure("scopes.worker.reallocations", 1),
            figure("scopes.worker.bytes_allocated", 192),
            figure("scopes.worker.bytes_freed", 128),
            figure("scopes.worker.live_at_end", 64),
            figure("scopes.worker.peak", 192),
            figure("scopes.worker.largest_entry_rise", 192),
            figure("scopes.worker.entries", 1),
        ]
    );
}
