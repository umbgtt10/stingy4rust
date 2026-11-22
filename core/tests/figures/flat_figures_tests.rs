// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::figures::figure::Figure;
use stingy4rust::figures::flat_figures::FlatFigures;
use stingy4rust::figures::heap_figures::HeapFigures;
use stingy4rust::figures::operation_figures::OperationFigures;
use stingy4rust::figures::scope_figures::ScopeFigures;
use stingy4rust::figures::scope_name::ScopeName;
use stingy4rust::figures::store_figures::StoreFigures;
use stingy4rust::figures::tally::Tally;
use stingy4rust::figures::wire_figures::WireFigures;
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
fn of_a_store_without_a_disk_writes_no_bytes_on_disk() {
    // Arrange
    let mut tally = tally(RunStatus::Measured, Vec::new());
    tally.storage = vec![StoreFigures {
        store: ScopeName {
            name: "memory",
            number: None,
        },
        bytes_written: 8,
        writes: 1,
        transactions: 0,
        bytes_at_rest: 8,
        bytes_on_disk: None,
    }];

    // Act
    let figures = FlatFigures::of(&tally);

    // Assert
    let paths: Vec<&str> = figures.iter().map(|figure| figure.path.as_str()).collect();
    assert!(paths.contains(&"storage.memory.bytes_at_rest"));
    assert!(!paths.contains(&"storage.memory.bytes_on_disk"));
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

#[test]
fn of_a_tally_lists_storage_wire_then_operations_after_the_scopes() {
    // Arrange
    let mut tally = tally(RunStatus::Measured, Vec::new());
    tally.storage = vec![StoreFigures {
        store: ScopeName {
            name: "log",
            number: Some(1),
        },
        bytes_written: 150,
        writes: 3,
        transactions: 2,
        bytes_at_rest: 150,
        bytes_on_disk: Some(4096),
    }];
    tally.wire = vec![WireFigures {
        kind: "ping",
        messages: 2,
        bytes: 20,
    }];
    tally.operations = vec![OperationFigures {
        name: "exchange",
        count: 2,
        messages: 4,
        bytes: 220,
    }];

    // Act
    let figures = FlatFigures::of(&tally);

    // Assert
    assert_eq!(
        figures[6..].to_vec(),
        vec![
            figure("storage.log 1.bytes_written", 150),
            figure("storage.log 1.writes", 3),
            figure("storage.log 1.transactions", 2),
            figure("storage.log 1.bytes_at_rest", 150),
            figure("storage.log 1.bytes_on_disk", 4096),
            figure("wire.ping.messages", 2),
            figure("wire.ping.bytes", 20),
            figure("operations.exchange.count", 2),
            figure("operations.exchange.messages", 4),
            figure("operations.exchange.bytes", 220),
        ]
    );
}
