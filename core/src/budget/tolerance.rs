// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// How far one figure may rise above its budget: by an absolute amount, by a
// share of the budget rounded up, or by the larger of the two; by nothing
// when it has neither. The sum stops at the largest budget there is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tolerance {
    absolute: Option<u64>,
    relative: Option<f64>,
}

impl Tolerance {
    #[must_use]
    pub const fn new(absolute: Option<u64>, relative: Option<f64>) -> Self {
        Self { absolute, relative }
    }

    #[must_use]
    pub const fn absolute(&self) -> Option<u64> {
        self.absolute
    }

    // The most a figure budgeted at `budget` may measure and stay within it.
    #[must_use]
    pub fn allowed(&self, budget: u64) -> u64 {
        let absolute = self.absolute.unwrap_or(0);
        let relative = self
            .relative
            .map_or(0, |relative| Self::share(budget, relative));
        budget.saturating_add(absolute.max(relative))
    }

    #[must_use]
    pub const fn relative(&self) -> Option<f64> {
        self.relative
    }

    // `ceil(budget * relative)`, without the `ceil` a `no_std` core lacks: the
    // product cast down, plus one if that lost a fraction. A cast saturates, so
    // a negative or not-a-number share is 0 and a huge one is the largest.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    fn share(budget: u64, relative: f64) -> u64 {
        let product = budget as f64 * relative;
        let whole = product as u64;
        if (whole as f64) < product {
            whole.saturating_add(1)
        } else {
            whole
        }
    }
}
