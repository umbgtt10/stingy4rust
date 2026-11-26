// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::report::json_writer::JsonWriter;

#[test]
fn into_text_of_an_empty_array_and_object_is_their_brackets() {
    // Arrange
    let writer = JsonWriter::new()
        .begin_object()
        .key("none")
        .begin_array()
        .end_array()
        .key("nothing")
        .begin_object()
        .end_object()
        .end_object();

    // Act
    let text = writer.into_text();

    // Assert
    assert_eq!(text, "{\n  \"none\": [],\n  \"nothing\": {}\n}");
}

#[test]
fn into_text_of_keys_and_values_writes_one_a_line_indented_by_depth() {
    // Arrange
    let writer = JsonWriter::new()
        .begin_object()
        .key("name")
        .string("three-blocks")
        .key("runs")
        .begin_array()
        .begin_object()
        .key("seed")
        .number(2026)
        .end_object()
        .number(7)
        .end_array()
        .end_object();

    // Act
    let text = writer.into_text();

    // Assert
    assert_eq!(
        text,
        "{\n  \"name\": \"three-blocks\",\n  \"runs\": [\n    {\n      \"seed\": 2026\n    },\n    7\n  ]\n}"
    );
}

#[test]
fn key_escapes_what_a_json_string_cannot_hold() {
    // Arrange
    let writer = JsonWriter::new().begin_object();

    // Act
    let writer = writer.key("scopes.a\"b.peak").number(1);

    // Assert
    assert_eq!(
        writer.end_object().into_text(),
        "{\n  \"scopes.a\\\"b.peak\": 1\n}"
    );
}

#[test]
fn string_escapes_a_control_character_past_0x0f_with_both_hex_digits() {
    // Arrange
    let writer = JsonWriter::new();

    // Act
    let writer = writer.string("\u{1f}");

    // Assert
    assert_eq!(writer.into_text(), "\"\\u001f\"");
}

#[test]
fn string_escapes_a_quote_a_backslash_and_control_characters() {
    // Arrange
    let writer = JsonWriter::new();

    // Act
    let writer = writer.string("a\"b\\c\nd\te\rf\u{1}");

    // Assert
    assert_eq!(writer.into_text(), "\"a\\\"b\\\\c\\nd\\te\\rf\\u0001\"");
}
