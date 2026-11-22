# ADR-CountingNeverAllocates

## Status

- **Status:** Accepted
- **Date:** 2026-11-21

## Context

The counting runs inside the global allocator. If it allocated, it would
recurse into itself and count its own work as the code's.

## Decision

Every path an allocator call reaches is free of allocation and of panics: fixed
tables sized by const generics -- `SCOPES`, `BLOCKS`, `NAMES` -- and a constant
nesting depth; names are `&'static str`, never built. A table that fills sets
the run's status (`ScopesExhausted`, `BlocksExhausted`, `NamesExhausted`,
`DepthExhausted`) and the gate reports it with exit 2 instead of a figure.
Building the tally and the report happens after the run ends, when allocating
is allowed.

## Forcing constraints / Evidence

Recursion in a global allocator is at best a deadlock and at worst a count that
includes itself. A board has no room for a table that grows.

## Rejected alternatives

- **Growable tables, allocated through the inner allocator directly**: still
  allocation inside the allocator, and memory that is not the code's in the
  figures of a board.
- **Silently dropping what does not fit**: a figure quietly wrong is worse than
  no figure.

## Consequences

A project whose runs hold more live blocks than the default names its own
`Stingy<System, S, B, N>`. The tables' size is reported in the bookkeeping.
