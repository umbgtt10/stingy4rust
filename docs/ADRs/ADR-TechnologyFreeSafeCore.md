# ADR-TechnologyFreeSafeCore

## Status

- **Status:** Accepted
- **Date:** 2026-11-21

## Context

The family keeps its analysis in a core crate that is `no_std`, forbids
`unsafe` and touches no technology, so it can be mutation-tested alone, with the
command line and the adapters in a second crate. stingy4rust's heap figures
need a `GlobalAlloc`, which is `unsafe` to implement; a profile target needs
`std` to read its environment and write its report; and the core must run on
microcontrollers.

## Decision

Three published crates. `stingy4rust-core` (library `stingy4rust`) is
`#![no_std]` with `alloc`, `#![forbid(unsafe_code)]`, and holds all the
counting -- the ledger, scopes, peaks, runs, meters, the report and its JSON
writer, budgets and the comparison -- as safe code; its one dependency is
`spin`, for a lock that needs neither `std` nor an allocation.
`stingy4rust-host` is the host's `GlobalAlloc` -- the only `unsafe` code, in one
file, handing each call to the ledger -- plus the thread mark and the profile
target's `main`. `cargo-stingy4rust` is the subcommand. A board crate joins them
in slice 6.

## Forcing constraints / Evidence

A `GlobalAlloc` cannot be implemented without `unsafe impl`; putting it in the
core would break the family's `forbid(unsafe_code)` for the code most worth
mutating. A profile target cannot depend on the subcommand's crate without
linking its command line into every project's tests.

## Rejected alternatives

- **The allocator in the core, `unsafe` allowed in one module**: the core's
  guarantee becomes a convention.
- **The allocator and the runner in the subcommand's library**: every project's
  profile target links clap and the subcommand's adapters.
- **A JSON crate in the core**: the report is written by the measured process
  after its runs end; a small writer of its own, keys in a fixed order, is
  enough and is deterministic by construction.

## Consequences

Projects take two dev-dependencies, the core and the host crate; the board
crate replaces the host crate on a board. The name `stingy4rust-host` (and
later `stingy4rust-board`) has to be reserved on crates.io before the first
release.
