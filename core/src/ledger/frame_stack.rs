// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::ledger::frame::Frame;
use crate::ledger::max_depth::MAX_DEPTH;

const EMPTY: Frame = Frame {
    slot: 0,
    base: 0,
    high: 0,
};

// The scope entries open now, innermost last, at most `MAX_DEPTH` of them: a
// fixed array, so opening one never allocates, and a full stack refuses the
// frame instead.
pub struct FrameStack {
    frames: [Frame; MAX_DEPTH],
    len: usize,
}

impl FrameStack {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            frames: [EMPTY; MAX_DEPTH],
            len: 0,
        }
    }

    pub const fn clear(&mut self) {
        self.len = 0;
    }

    #[must_use]
    pub const fn current(&self) -> Option<usize> {
        match self.len.checked_sub(1) {
            Some(top) => Some(self.frames[top].slot),
            None => None,
        }
    }

    // How many entries are open now.
    #[must_use]
    pub const fn depth(&self) -> usize {
        self.len
    }

    pub const fn pop(&mut self) -> Option<Frame> {
        match self.len.checked_sub(1) {
            Some(top) => {
                self.len = top;
                Some(self.frames[top])
            }
            None => None,
        }
    }

    pub const fn push(&mut self, frame: Frame) -> Option<usize> {
        if self.len == MAX_DEPTH {
            return None;
        }
        self.frames[self.len] = frame;
        self.len += 1;
        Some(self.len)
    }

    // A slot's bytes live rose to `live`: every open entry of that slot has
    // seen at least that much.
    pub fn raise(&mut self, slot: usize, live: u64) {
        for frame in &mut self.frames[..self.len] {
            if frame.slot == slot {
                frame.high = frame.high.max(live);
            }
        }
    }
}

impl Default for FrameStack {
    fn default() -> Self {
        Self::new()
    }
}
