// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fmt;

use serde::Deserialize;
use serde::Deserializer;
use serde::de::MapAccess;
use serde::de::Visitor;
use stingy4rust::figures::figure::Figure;

// A report's figures read in the order they were written -- which a map read
// into a sorted or hashed collection would lose -- each a whole number.
pub struct FigureList {
    figures: Vec<Figure>,
}

impl FigureList {
    #[must_use]
    pub fn into_figures(self) -> Vec<Figure> {
        self.figures
    }
}

impl<'de> Deserialize<'de> for FigureList {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(Self {
            figures: Vec::new(),
        })
    }
}

impl<'de> Visitor<'de> for FigureList {
    type Value = Self;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "an object of figures, each a whole number")
    }

    fn visit_map<A: MapAccess<'de>>(mut self, mut map: A) -> Result<Self::Value, A::Error> {
        while let Some((path, value)) = map.next_entry::<String, u64>()? {
            self.figures.push(Figure { path, value });
        }
        Ok(self)
    }
}
