// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde_json::from_str;
use stingy4rust::figures::figure::Figure;
use stingy4rust_cli::formats::figure_list::FigureList;

fn figure(path: &str, value: u64) -> Figure {
    Figure {
        path: String::from(path),
        value,
    }
}

#[test]
fn deserialize_of_a_figure_that_is_not_a_whole_number_fails() {
    // Arrange
    let text = r#"{"peaks.run": 1.5}"#;

    // Act
    let list = from_str::<FigureList>(text);

    // Assert
    assert!(list.is_err());
    assert!(from_str::<FigureList>(r#"{"peaks.run": 1}"#).is_ok());
}

#[test]
fn deserialize_of_an_object_keeps_its_keys_in_the_order_written() {
    // Arrange
    let text = r#"{"peaks.run": 192, "heap.allocations": 3, "scopes.(outside scopes).entries": 0}"#;

    // Act
    let list: FigureList = from_str(text).expect("figures");

    // Assert
    assert_eq!(
        list.into_figures(),
        vec![
            figure("peaks.run", 192),
            figure("heap.allocations", 3),
            figure("scopes.(outside scopes).entries", 0),
        ]
    );
}
