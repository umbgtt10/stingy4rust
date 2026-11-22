// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

const INDENT: &str = "  ";

// JSON written a token at a time, in the order it is given, so the same calls
// write the same bytes: one element a line, indented two spaces a level, and
// every string escaped as JSON requires. A builder, taken and handed back by
// each call, so the functions that write one part of a document return what
// they wrote.
pub struct JsonWriter {
    text: String,
    open: Vec<bool>,
    after_key: bool,
}

impl JsonWriter {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            text: String::new(),
            open: Vec::new(),
            after_key: false,
        }
    }

    #[must_use]
    pub fn begin_array(self) -> Self {
        self.opened('[')
    }

    #[must_use]
    pub fn begin_object(self) -> Self {
        self.opened('{')
    }

    #[must_use]
    pub fn end_array(self) -> Self {
        self.closed(']')
    }

    #[must_use]
    pub fn end_object(self) -> Self {
        self.closed('}')
    }

    #[must_use]
    pub fn into_text(self) -> String {
        self.text
    }

    #[must_use]
    pub fn key(self, key: &str) -> Self {
        let mut writer = self.on_next_line().quoted(key);
        writer.text.push_str(": ");
        writer.after_key = true;
        writer
    }

    #[must_use]
    pub fn number(self, value: u64) -> Self {
        let mut writer = self.at_value();
        writer.text.push_str(&value.to_string());
        writer
    }

    #[must_use]
    pub fn string(self, value: &str) -> Self {
        self.at_value().quoted(value)
    }

    // Where a value goes: straight after its key, or on a line of its own in
    // an array.
    fn at_value(mut self) -> Self {
        if self.after_key {
            self.after_key = false;
            self
        } else if self.open.is_empty() {
            self
        } else {
            self.on_next_line()
        }
    }

    fn closed(mut self, bracket: char) -> Self {
        if self.open.pop() == Some(true) {
            self.text.push('\n');
            self.text.push_str(&INDENT.repeat(self.open.len()));
        }
        self.text.push(bracket);
        self
    }

    // A comma after the element before, if there was one, then a new line
    // indented to the depth open now.
    fn on_next_line(mut self) -> Self {
        if let Some(has_elements) = self.open.last_mut() {
            if *has_elements {
                self.text.push(',');
            }
            *has_elements = true;
        }
        self.text.push('\n');
        self.text.push_str(&INDENT.repeat(self.open.len()));
        self
    }

    fn opened(self, bracket: char) -> Self {
        let mut writer = self.at_value();
        writer.text.push(bracket);
        writer.open.push(false);
        writer
    }

    fn quoted(mut self, value: &str) -> Self {
        self.text.push('"');
        for character in value.chars() {
            match character {
                '"' => self.text.push_str("\\\""),
                '\\' => self.text.push_str("\\\\"),
                '\n' => self.text.push_str("\\n"),
                '\r' => self.text.push_str("\\r"),
                '\t' => self.text.push_str("\\t"),
                control if control < ' ' => {
                    let code = u32::from(control);
                    self.text.push_str("\\u00");
                    for digit in [code >> 4, code & 0xf] {
                        self.text.push(char::from_digit(digit, 16).unwrap_or('0'));
                    }
                }
                other => self.text.push(other),
            }
        }
        self.text.push('"');
        self
    }
}

impl Default for JsonWriter {
    fn default() -> Self {
        Self::new()
    }
}
