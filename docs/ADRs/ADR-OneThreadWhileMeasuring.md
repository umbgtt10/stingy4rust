# ADR-OneThreadWhileMeasuring

## Status

- **Status:** Accepted
- **Date:** 2026-11-21

## Context

Scopes attribute allocations to the code running in them. With two threads
allocating, an allocation lands in whichever scope is open, and the order of
two threads differs from run to run.

## Decision

A run is measured on one thread. `begin` records the calling thread's mark;
while a scope is open, an allocator call from another thread sets the run's
`ForeignThread` status and is not counted. On a host the mark is the address of
a `const`-initialised thread-local; on a board, a constant.

## Forcing constraints / Evidence

The figures are a gate only if they repeat, and an interleaving of threads does
not repeat. The projects this is built for already run their simulated systems
on one thread, which is what makes them deterministic.

## Rejected alternatives

- **A scope stack per thread**: attributes correctly, but the figures still
  vary with scheduling.
- **Ignoring other threads silently**: hides allocations the code made.

## Consequences

A profile that starts threads fails with exit 2. A process may still allocate
on other threads while no scope is open -- before and after the runs.
