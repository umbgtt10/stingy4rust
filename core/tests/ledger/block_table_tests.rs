// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::ledger::block_entry::BlockEntry;
use stingy4rust::ledger::block_table::BlockTable;

const fn block(address: usize) -> BlockEntry {
    BlockEntry {
        address,
        size: 8,
        scope: 1,
    }
}

fn holding(addresses: &[usize]) -> BlockTable<8> {
    let mut table = BlockTable::new();
    for address in addresses {
        table.insert(block(*address));
    }
    table
}

#[test]
fn clear_of_a_table_holding_blocks_forgets_them_and_its_peak() {
    // Arrange
    let mut table = holding(&[0x1000, 0x1010]);

    // Act
    table.clear();

    // Assert
    assert_eq!(table.remove(0x1010), None);
    assert!(table.insert(block(0x1000)));
    assert_eq!(table.tracked(), 1);
    assert_eq!(table.peak(), 1);
}

#[test]
fn insert_into_a_full_table_reports_it_full() {
    // Arrange
    let mut table = holding(&[
        0x1000, 0x1010, 0x1020, 0x1030, 0x1040, 0x1050, 0x1060, 0x1070,
    ]);

    // Act
    let inserted = table.insert(block(0x1080));

    // Assert
    assert!(!inserted);
    assert_eq!(table.tracked(), 8);
}

#[test]
fn insert_into_a_table_of_one_block_holds_it() {
    // Arrange
    let mut table = BlockTable::<1>::new();

    // Act
    let inserted = table.insert(block(0x1000));

    // Assert
    assert!(inserted);
    assert_eq!(table.remove(0x1000), Some(block(0x1000)));
}

#[test]
fn insert_then_remove_leaves_the_table_empty() {
    // Arrange
    let mut table = BlockTable::<8>::new();

    // Act
    let inserted = table.insert(block(0x1000));
    let removed = table.remove(0x1000);

    // Assert
    assert!(inserted);
    assert_eq!(removed, Some(block(0x1000)));
    assert_eq!(table.tracked(), 0);
}

#[test]
fn peak_reports_the_most_blocks_held_at_once() {
    // Arrange
    let mut table = holding(&[0x1000, 0x1010, 0x1020]);
    table.remove(0x1000);
    table.remove(0x1010);
    table.insert(block(0x1030));

    // Act
    let peak = table.peak();

    // Assert
    assert_eq!(peak, 3);
}

#[test]
fn remove_from_a_full_table_of_an_address_it_lacks_finds_nothing() {
    // Arrange
    let mut table = holding(&[
        0x1000, 0x1010, 0x1020, 0x1030, 0x1040, 0x1050, 0x1060, 0x1070,
    ]);

    // Act
    let removed = table.remove(0x2000);

    // Assert
    assert_eq!(removed, None);
    assert_eq!(table.tracked(), 8);
}

// 0x1000 and 0x1080 hash to slot 1, 0x1020 to slot 3, where it sits. Removing
// 0x1000 shifts 0x1080 back into slot 1; 0x1020 is at its home and must stay.
#[test]
fn remove_keeps_a_block_already_at_its_home_where_it_is() {
    // Arrange
    let mut table = holding(&[0x1000, 0x1080, 0x1020]);

    // Act
    let removed = table.remove(0x1000);

    // Assert
    assert_eq!(removed, Some(block(0x1000)));
    assert_eq!(table.remove(0x1020), Some(block(0x1020)));
    assert_eq!(table.remove(0x1080), Some(block(0x1080)));
}

// 0x1060, 0x1130 and 0x11b0 all hash to slot 7, the last, so the chain wraps
// to slots 0 and 1.
#[test]
fn remove_of_a_block_in_a_chain_wrapping_past_the_end_keeps_the_others_found() {
    // Arrange
    let mut table = holding(&[0x1060, 0x1130, 0x11b0]);

    // Act
    let removed = table.remove(0x1060);

    // Assert
    assert_eq!(removed, Some(block(0x1060)));
    assert_eq!(table.remove(0x11b0), Some(block(0x11b0)));
    assert_eq!(table.remove(0x1130), Some(block(0x1130)));
}

// 0x1000, 0x1080 and 0x1150 all hash to slot 1, so they sit in slots 1, 2, 3.
#[test]
fn remove_of_a_block_in_a_probe_chain_keeps_the_others_found() {
    // Arrange
    let mut table = holding(&[0x1000, 0x1080, 0x1150]);

    // Act
    let removed = table.remove(0x1080);

    // Assert
    assert_eq!(removed, Some(block(0x1080)));
    assert_eq!(table.remove(0x1150), Some(block(0x1150)));
    assert_eq!(table.remove(0x1000), Some(block(0x1000)));
    assert_eq!(table.tracked(), 0);
}

#[test]
fn remove_of_an_address_never_inserted_finds_nothing() {
    // Arrange
    let mut table = holding(&[0x1000]);

    // Act
    let removed = table.remove(0x2000);

    // Assert
    assert_eq!(removed, None);
    assert_eq!(table.tracked(), 1);
}
