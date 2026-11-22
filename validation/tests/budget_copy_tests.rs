// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde_json::Value;
use serde_json::from_str;
use std::fs::read_to_string;
use std::path::Path;
use validation::budget_copy::BudgetCopy;

fn read(copy: &BudgetCopy) -> Value {
    from_str(&read_to_string(copy.path()).expect("the copy")).expect("JSON")
}

#[test]
fn absent_names_a_file_not_there() {
    // Arrange & Act
    let copy = BudgetCopy::absent();

    // Assert
    assert!(!Path::new(&copy.path()).exists());
    assert!(copy.path().ends_with("stingy4rust-budget.json"));
}

#[test]
fn corrupted_is_not_json() {
    // Arrange & Act
    let copy = BudgetCopy::of_fixture().corrupted();

    // Assert
    assert!(from_str::<Value>(&read_to_string(copy.path()).expect("the copy")).is_err());
}

#[test]
fn with_figure_of_a_value_sets_it_under_every_target() {
    // Arrange
    let copy = BudgetCopy::of_fixture();

    // Act
    let copy = copy.with_figure("three-blocks", "heap.allocations", Some(2));

    // Assert
    let budget = read(&copy);
    let targets = budget["targets"].as_object().expect("targets");
    assert!(!targets.is_empty());
    for profiles in targets.values() {
        assert_eq!(profiles["three-blocks"]["figures"]["heap.allocations"], 2);
    }
}

#[test]
fn with_figure_of_none_removes_it_under_every_target() {
    // Arrange
    let copy = BudgetCopy::of_fixture();

    // Act
    let copy = copy.with_figure("three-blocks", "peaks.run", None);

    // Assert
    let budget = read(&copy);
    let targets = budget["targets"].as_object().expect("targets");
    assert!(!targets.is_empty());
    for profiles in targets.values() {
        assert_eq!(profiles["three-blocks"]["figures"].get("peaks.run"), None);
        assert_eq!(profiles["three-blocks"]["figures"]["heap.allocations"], 3);
    }
}
