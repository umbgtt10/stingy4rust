# ADR-FiguresAreFlatPaths

## Status

- **Status:** Accepted
- **Date:** 2026-11-21

## Context

A run's figures are nested -- scopes, stores, kinds of message, operations --
and a budget must name each one, carry a tolerance for some, and be compared
figure by figure.

## Decision

Every figure has a path, its parts joined by `.`: `heap.allocations`,
`peaks.run`, `scopes.node 3.peak`, `storage.node 1 log.bytes_written`,
`wire.AppendEntries.bytes`, `operations.command.messages`. The report and the
budget hold flat maps from path to value; a budget is keyed by target triple,
then profile. A name holding a `.` is an error. Every figure measured must be
budgeted, and every figure budgeted must be measured.

## Forcing constraints / Evidence

A flat map makes the comparison one loop over paths, a tolerance one entry
beside its figure, and a re-recording's diff one line a figure. The same profile
gives different figures on a 32-bit board and a 64-bit host, so the target is
part of the key.

## Rejected alternatives

- **Nested JSON mirroring the figures**: a comparison and a tolerance per shape.
- **Ignoring figures with no budget**: a new scope or store would never be
  gated.

## Consequences

A scope that stops being entered fails as `missing` until the budget is
re-recorded, deliberately.
