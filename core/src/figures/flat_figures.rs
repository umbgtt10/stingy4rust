// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::figures::figure::Figure;
use crate::figures::heap_figures::HeapFigures;
use crate::figures::scope_figures::ScopeFigures;
use crate::figures::scope_name::ScopeName;
use crate::figures::tally::Tally;

// A tally as paths and values, in the design's order: the run's heap, its
// peak, then each scope's figures, the scopes in the order first entered. A
// numbered name is written with its number: `scopes.node 3.peak`.
pub struct FlatFigures;

impl FlatFigures {
    #[must_use]
    pub fn of(tally: &Tally) -> Vec<Figure> {
        let mut figures = Vec::from(Self::heap("heap", tally.heap));
        figures.push(Self::figure("peaks", "run", tally.peak));
        for scope in &tally.scopes {
            figures.extend(Self::scope(scope));
        }
        figures
    }

    fn figure(prefix: &str, name: &str, value: u64) -> Figure {
        Figure {
            path: format!("{prefix}.{name}"),
            value,
        }
    }

    fn heap(prefix: &str, heap: HeapFigures) -> [Figure; 5] {
        [
            Self::figure(prefix, "allocations", heap.allocations),
            Self::figure(prefix, "reallocations", heap.reallocations),
            Self::figure(prefix, "bytes_allocated", heap.bytes_allocated),
            Self::figure(prefix, "bytes_freed", heap.bytes_freed),
            Self::figure(prefix, "live_at_end", heap.live_at_end),
        ]
    }

    fn named(name: ScopeName) -> String {
        name.number.map_or_else(
            || String::from(name.name),
            |number| format!("{} {number}", name.name),
        )
    }

    fn scope(scope: &ScopeFigures) -> impl Iterator<Item = Figure> {
        let prefix = format!("scopes.{}", Self::named(scope.name));
        let rest = [
            Self::figure(&prefix, "peak", scope.peak),
            Self::figure(&prefix, "largest_entry_rise", scope.largest_entry_rise),
            Self::figure(&prefix, "entries", scope.entries),
        ];
        Self::heap(&prefix, scope.heap).into_iter().chain(rest)
    }
}
