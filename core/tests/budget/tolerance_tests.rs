// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::budget::tolerance::Tolerance;

#[test]
fn absolute_and_relative_are_what_it_was_built_with() {
    // Arrange
    let tolerance = Tolerance::new(Some(4096), Some(0.25));

    // Act
    let parts = (tolerance.absolute(), tolerance.relative());

    // Assert
    assert_eq!(parts, (Some(4096), Some(0.25)));
}

#[test]
fn allowed_near_the_largest_budget_stops_there() {
    // Arrange
    let tolerance = Tolerance::new(Some(10), None);

    // Act
    let allowed = tolerance.allowed(u64::MAX - 1);

    // Assert
    assert_eq!(allowed, u64::MAX);
}

#[test]
fn allowed_with_a_relative_tolerance_rounds_up() {
    // Arrange
    let tolerance = Tolerance::new(None, Some(0.05));

    // Act
    let allowed = tolerance.allowed(101);

    // Assert
    assert_eq!(allowed, 107);
}

#[test]
fn allowed_with_an_absolute_tolerance_adds_it() {
    // Arrange
    let tolerance = Tolerance::new(Some(4096), None);

    // Act
    let allowed = tolerance.allowed(100);

    // Assert
    assert_eq!(allowed, 4196);
}

#[test]
fn allowed_with_an_exact_relative_product_adds_no_more() {
    // Arrange
    let tolerance = Tolerance::new(None, Some(0.5));

    // Act
    let allowed = tolerance.allowed(100);

    // Assert
    assert_eq!(allowed, 150);
}

#[test]
fn allowed_with_both_takes_the_larger() {
    // Arrange
    let absolute_larger = Tolerance::new(Some(10), Some(0.05));
    let relative_larger = Tolerance::new(Some(2), Some(0.05));

    // Act
    let allowed = [absolute_larger.allowed(100), relative_larger.allowed(100)];

    // Assert
    assert_eq!(allowed, [110, 105]);
}

#[test]
fn allowed_with_no_tolerance_is_the_budget() {
    // Arrange
    let tolerance = Tolerance::new(None, None);

    // Act
    let allowed = tolerance.allowed(192);

    // Assert
    assert_eq!(allowed, 192);
}
