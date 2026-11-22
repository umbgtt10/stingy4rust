// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::figures::heap_figures::HeapFigures;
use crate::figures::scope_figures::ScopeFigures;
use crate::figures::scope_name::ScopeName;

const EMPTY: ScopeFigures = ScopeFigures {
    name: ScopeName {
        name: "",
        number: None,
    },
    heap: HeapFigures {
        allocations: 0,
        reallocations: 0,
        bytes_allocated: 0,
        bytes_freed: 0,
        live_at_end: 0,
    },
    peak: 0,
    largest_entry_rise: 0,
    entries: 0,
};
const OUTSIDE: ScopeName = ScopeName {
    name: "(outside scopes)",
    number: None,
};

// A run's scopes and what each has cost so far: slot 0 is `(outside scopes)`,
// the rest are named in the order first entered, in a fixed array, so naming
// one never allocates and a full table refuses the name. Every count
// saturates; a slot it does not hold is ignored.
pub struct ScopeTable<const SCOPES: usize> {
    slots: [ScopeFigures; SCOPES],
    len: usize,
}

impl<const SCOPES: usize> ScopeTable<SCOPES> {
    #[must_use]
    pub const fn new() -> Self {
        const { assert!(SCOPES > 0, "SCOPES must hold (outside scopes)") };
        let mut slots = [EMPTY; SCOPES];
        slots[0] = Self::fresh(OUTSIDE);
        Self { slots, len: 1 }
    }

    // Charges `size` to the slot as one allocation and hands back what it holds
    // live now.
    pub fn allocated(&mut self, slot: usize, size: u64) -> u64 {
        self.charge(slot, size, 1, 0)
    }

    pub const fn clear(&mut self) {
        self.slots[0] = Self::fresh(OUTSIDE);
        self.len = 1;
    }

    pub fn entered(&mut self, slot: usize) {
        if let Some(figures) = self.slots[..self.len].get_mut(slot) {
            figures.entries = figures.entries.saturating_add(1);
        }
    }

    #[must_use]
    pub fn figures(&self) -> Vec<ScopeFigures> {
        self.slots[..self.len].to_vec()
    }

    pub fn freed(&mut self, slot: usize, size: u64) {
        if let Some(figures) = self.slots[..self.len].get_mut(slot) {
            figures.heap.bytes_freed = figures.heap.bytes_freed.saturating_add(size);
            figures.heap.live_at_end = figures.heap.live_at_end.saturating_sub(size);
        }
    }

    // The slot of a name: the one it was given when first met, or the next
    // free one; none once the table is full.
    pub fn intern(&mut self, name: ScopeName) -> Option<usize> {
        if let Some(slot) = self.slots[..self.len]
            .iter()
            .position(|figures| figures.name == name)
        {
            return Some(slot);
        }
        let slot = self.len;
        *self.slots.get_mut(slot)? = Self::fresh(name);
        self.len += 1;
        Some(slot)
    }

    #[must_use]
    pub fn live(&self, slot: usize) -> u64 {
        self.slots[..self.len]
            .get(slot)
            .map_or(0, |figures| figures.heap.live_at_end)
    }

    // Charges `size` to the slot as one reallocation and hands back what it
    // holds live now; the old block's size is freed apart, from its own slot.
    pub fn reallocated(&mut self, slot: usize, size: u64) -> u64 {
        self.charge(slot, size, 0, 1)
    }

    pub fn rose(&mut self, slot: usize, rise: u64) {
        if let Some(figures) = self.slots[..self.len].get_mut(slot) {
            figures.largest_entry_rise = figures.largest_entry_rise.max(rise);
        }
    }

    fn charge(&mut self, slot: usize, size: u64, allocations: u64, reallocations: u64) -> u64 {
        let Some(figures) = self.slots[..self.len].get_mut(slot) else {
            return 0;
        };
        let live = figures.heap.live_at_end.saturating_add(size);
        *figures = ScopeFigures {
            heap: HeapFigures {
                allocations: figures.heap.allocations.saturating_add(allocations),
                reallocations: figures.heap.reallocations.saturating_add(reallocations),
                bytes_allocated: figures.heap.bytes_allocated.saturating_add(size),
                live_at_end: live,
                ..figures.heap
            },
            peak: figures.peak.max(live),
            ..*figures
        };
        live
    }

    const fn fresh(name: ScopeName) -> ScopeFigures {
        ScopeFigures { name, ..EMPTY }
    }
}

impl<const SCOPES: usize> Default for ScopeTable<SCOPES> {
    fn default() -> Self {
        Self::new()
    }
}
