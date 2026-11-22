// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::budget::verdict::Verdict;

// A verdict as the table and the JSON both write it.
pub struct VerdictName;

impl VerdictName {
    #[must_use]
    pub const fn of(verdict: Verdict) -> &'static str {
        match verdict {
            Verdict::Within => "within",
            Verdict::Gain => "gain",
            Verdict::Over => "over",
            Verdict::Unbudgeted => "unbudgeted",
            Verdict::Missing => "missing",
        }
    }
}
