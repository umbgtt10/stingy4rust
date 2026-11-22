// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// How one figure stands against its budget: within it -- equal, or risen no
// further than its tolerance --, a gain below it, over it, measured with no
// budget, or budgeted and not measured. The last three fail a check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Within,
    Gain,
    Over,
    Unbudgeted,
    Missing,
}

impl Verdict {
    #[must_use]
    pub const fn fails(self) -> bool {
        matches!(self, Self::Over | Self::Unbudgeted | Self::Missing)
    }
}
