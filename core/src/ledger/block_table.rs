// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::ledger::block_entry::BlockEntry;

const MULTIPLIER: u64 = 0x9E37_79B9_7F4A_7C15;

// The live blocks of a run, by address: open addressing in a fixed array whose
// capacity is a power of two, linear probing from a multiplicative hash of the
// address, and backward-shift deletion, so no tombstone ever fills it. Nothing
// here allocates or panics: a full table refuses the block instead.
pub struct BlockTable<const BLOCKS: usize> {
    entries: [Option<BlockEntry>; BLOCKS],
    len: usize,
    peak: usize,
}

impl<const BLOCKS: usize> BlockTable<BLOCKS> {
    #[must_use]
    pub const fn new() -> Self {
        const { assert!(BLOCKS.is_power_of_two(), "BLOCKS must be a power of two") };
        Self {
            entries: [None; BLOCKS],
            len: 0,
            peak: 0,
        }
    }

    pub fn clear(&mut self) {
        self.entries.fill(None);
        self.len = 0;
        self.peak = 0;
    }

    pub fn insert(&mut self, entry: BlockEntry) -> bool {
        if self.len == BLOCKS {
            return false;
        }
        let mut index = Self::home(entry.address);
        while self.entries[index].is_some() {
            index = Self::next(index);
        }
        self.entries[index] = Some(entry);
        self.len += 1;
        self.peak = self.peak.max(self.len);
        true
    }

    #[must_use]
    pub const fn peak(&self) -> usize {
        self.peak
    }

    pub fn remove(&mut self, address: usize) -> Option<BlockEntry> {
        let mut index = Self::home(address);
        for _ in 0..BLOCKS {
            let entry = self.entries[index]?;
            if entry.address == address {
                self.entries[index] = None;
                self.len -= 1;
                self.shift_back(index);
                return Some(entry);
            }
            index = Self::next(index);
        }
        None
    }

    // How many blocks the table holds now.
    #[must_use]
    pub const fn tracked(&self) -> usize {
        self.len
    }

    fn home(address: usize) -> usize {
        let hash = ((address as u64) >> 4).wrapping_mul(MULTIPLIER);
        let slot = hash.checked_shr(64 - BLOCKS.trailing_zeros()).unwrap_or(0);
        usize::try_from(slot).unwrap_or(0)
    }

    const fn next(index: usize) -> usize {
        (index + 1) & (BLOCKS - 1)
    }

    // Closes the hole at `hole`: each block after it in the run moves back into
    // it if the hole lies on that block's probe path -- from its home to where
    // it sits -- so every block stays reachable from its home.
    fn shift_back(&mut self, hole: usize) {
        let mut hole = hole;
        let mut index = Self::next(hole);
        for _ in 1..BLOCKS {
            let Some(entry) = self.entries[index] else {
                return;
            };
            let travelled = index.wrapping_sub(Self::home(entry.address)) & (BLOCKS - 1);
            let behind_hole = index.wrapping_sub(hole) & (BLOCKS - 1);
            if travelled >= behind_hole {
                self.entries[hole] = Some(entry);
                self.entries[index] = None;
                hole = index;
            }
            index = Self::next(index);
        }
    }
}

impl<const BLOCKS: usize> Default for BlockTable<BLOCKS> {
    fn default() -> Self {
        Self::new()
    }
}
