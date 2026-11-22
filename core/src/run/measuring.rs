// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::figures::tally::Tally;
use crate::run::recorder::Recorder;

// A recorder that also runs the counting: from `begin` to `end`, which hands
// back what was counted. A host's counting allocator is one; a board's is
// another.
pub trait Measuring: Recorder {
    fn begin(&self);
    fn end(&self) -> Tally;
}
