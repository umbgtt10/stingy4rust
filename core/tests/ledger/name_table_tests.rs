// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::figures::operation_figures::OperationFigures;
use stingy4rust::figures::scope_name::ScopeName;
use stingy4rust::figures::store_figures::StoreFigures;
use stingy4rust::figures::wire_figures::WireFigures;
use stingy4rust::ledger::name_event::NameEvent;
use stingy4rust::ledger::name_role::NameRole;
use stingy4rust::ledger::name_table::NameTable;

const fn named(name: &'static str) -> ScopeName {
    ScopeName { name, number: None }
}

const fn store(name: &'static str, bytes_written: u64, writes: u64) -> StoreFigures {
    StoreFigures {
        store: named(name),
        bytes_written,
        writes,
        transactions: 0,
        bytes_at_rest: 0,
        bytes_on_disk: None,
    }
}

#[test]
fn clear_forgets_every_name() {
    // Arrange
    let mut table = NameTable::<4>::new();
    let ping = table.intern(NameRole::Kind, named("ping")).expect("a slot");
    table.record(ping, NameEvent::Counted(10));

    // Act
    table.clear();

    // Assert
    assert_eq!(table.wire(), Vec::new());
    assert_eq!(table.intern(NameRole::Kind, named("pong")), Some(0));
}

#[test]
fn intern_of_a_name_met_before_is_its_slot() {
    // Arrange
    let mut table = NameTable::<4>::new();
    let first = table.intern(NameRole::Kind, named("ping"));

    // Act
    let again = table.intern(NameRole::Kind, named("ping"));

    // Assert
    assert_eq!((first, again), (Some(0), Some(0)));
}

#[test]
fn intern_of_one_name_in_two_roles_takes_two_slots() {
    // Arrange
    let mut table = NameTable::<4>::new();
    table.intern(NameRole::Kind, named("log"));

    // Act
    let store = table.intern(NameRole::Store, named("log"));

    // Assert
    assert_eq!(store, Some(1));
}

#[test]
fn intern_past_the_capacity_returns_none() {
    // Arrange
    let mut table = NameTable::<2>::new();

    // Act
    let slots = [
        table.intern(NameRole::Kind, named("ping")),
        table.intern(NameRole::Kind, named("pong")),
        table.intern(NameRole::Kind, named("one too many")),
    ];

    // Assert
    assert_eq!(slots, [Some(0), Some(1), None]);
}

#[test]
fn operations_lists_each_operation_with_its_messages_and_bytes() {
    // Arrange
    let mut table = NameTable::<4>::new();
    let exchange = table
        .intern(NameRole::Operation, named("exchange"))
        .expect("a slot");
    table.record(exchange, NameEvent::Counted(110));
    table.record(exchange, NameEvent::Completed);

    // Act
    let operations = table.operations();

    // Assert
    assert_eq!(
        operations,
        vec![OperationFigures {
            name: "exchange",
            count: 1,
            messages: 1,
            bytes: 110,
        }]
    );
}

#[test]
fn record_at_rest_keeps_the_last_value() {
    // Arrange
    let mut table = NameTable::<4>::new();
    let journal = table
        .intern(NameRole::Store, named("journal"))
        .expect("a slot");

    // Act
    table.record(journal, NameEvent::AtRest(300));
    table.record(journal, NameEvent::AtRest(150));

    // Assert
    assert_eq!(table.storage()[0].bytes_at_rest, 150);
}

#[test]
fn record_completed_counts_an_operation_or_a_transaction() {
    // Arrange
    let mut table = NameTable::<4>::new();
    let exchange = table
        .intern(NameRole::Operation, named("exchange"))
        .expect("a slot");
    let journal = table
        .intern(NameRole::Store, named("journal"))
        .expect("a slot");

    // Act
    table.record(exchange, NameEvent::Completed);
    table.record(exchange, NameEvent::Completed);
    table.record(journal, NameEvent::Completed);

    // Assert
    assert_eq!(table.operations()[0].count, 2);
    assert_eq!(table.storage()[0].transactions, 1);
}

#[test]
fn record_counted_on_a_kind_is_a_message_and_on_a_store_a_write() {
    // Arrange
    let mut table = NameTable::<4>::new();
    let ping = table.intern(NameRole::Kind, named("ping")).expect("a slot");
    let journal = table
        .intern(NameRole::Store, named("journal"))
        .expect("a slot");

    // Act
    table.record(ping, NameEvent::Counted(10));
    table.record(ping, NameEvent::Counted(10));
    table.record(journal, NameEvent::Counted(50));

    // Assert
    assert_eq!(
        table.wire(),
        vec![WireFigures {
            kind: "ping",
            messages: 2,
            bytes: 20,
        }]
    );
    assert_eq!(table.storage(), vec![store("journal", 50, 1)]);
}

#[test]
fn record_on_a_slot_out_of_range_changes_nothing() {
    // Arrange
    let mut table = NameTable::<4>::new();
    let ping = table.intern(NameRole::Kind, named("ping")).expect("a slot");

    // Act
    table.record(ping + 3, NameEvent::Counted(10));

    // Assert
    assert_eq!(
        table.wire(),
        vec![WireFigures {
            kind: "ping",
            messages: 0,
            bytes: 0,
        }]
    );
}

#[test]
fn record_on_disk_reports_a_store_s_files_and_a_store_without_reports_none() {
    // Arrange
    let mut table = NameTable::<4>::new();
    let journal = table
        .intern(NameRole::Store, named("journal"))
        .expect("a slot");
    table.intern(NameRole::Store, named("memory"));

    // Act
    table.record(journal, NameEvent::OnDisk(4096));

    // Assert
    let storage = table.storage();
    assert_eq!(storage[0].bytes_on_disk, Some(4096));
    assert_eq!(storage[1].bytes_on_disk, None);
}

#[test]
fn storage_lists_stores_in_the_order_first_recorded_with_their_numbers() {
    // Arrange
    let mut table = NameTable::<4>::new();
    table.intern(
        NameRole::Store,
        ScopeName {
            name: "log",
            number: Some(2),
        },
    );
    table.intern(NameRole::Kind, named("ping"));
    table.intern(NameRole::Store, named("journal"));

    // Act
    let storage = table.storage();

    // Assert
    let names: Vec<ScopeName> = storage.iter().map(|figures| figures.store).collect();
    assert_eq!(
        names,
        vec![
            ScopeName {
                name: "log",
                number: Some(2)
            },
            named("journal")
        ]
    );
}
