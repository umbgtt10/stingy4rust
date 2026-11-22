# ADR-ToleranceIsZeroAndPerFigure

## Status

- **Status:** Accepted
- **Date:** 2026-11-21

## Context

A budget is compared with what a run measures. If a figure may rise a little
without failing, small regressions pass; if it may not, any noise fails the
gate.

## Decision

The tolerance is zero by default: a figure above its budget fails. A figure
may carry its own tolerance in the budget file -- `absolute`, `relative` or both,
allowing `budget + max(absolute, ceil(budget * relative))` -- and there is no
tolerance for a whole profile, target or file. A figure below its budget passes
and is reported as a gain.

## Forcing constraints / Evidence

The owner's answer of 2026-11-21. The runs are deterministic (one thread, a
fixed seed, a pinned toolchain, release builds), so a figure repeats exactly
and any growth is a change in the code. A blanket margin of even 1% admits a
regression a percent at a time, each one under the bar.

## Rejected alternatives

- **A small blanket margin**: absorbs a toolchain bump without re-recording,
  and every slow regression with it.
- **Failing on gains too**: makes a saving a red gate; a gain is reported and
  recorded deliberately instead.

## Consequences

A figure that genuinely varies -- a store's size on disk, which depends on its
engine's pages -- carries a tolerance on its own line, and the commit that adds
it says why. A toolchain bump re-records the budgets in its own commit.
