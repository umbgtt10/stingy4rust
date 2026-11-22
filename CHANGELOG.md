# Changelog

All notable changes to `cargo-stingy4rust` and its crates are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

## [0.1.0] - 2026-11-22

The host tool: a project's load profiles run under a counting allocator, their
heap, peaks, storage and wire figures reported, held to a budget and recorded
as one, from `cargo stingy4rust`. Microcontrollers come later; the board crate
still only reserves its name.

### Added

- The ledger (`stingy4rust::ledger`): a run's heap counted by scope in fixed
  tables behind a `spin` lock -- allocations, reallocations, bytes allocated,
  freed and live, the peak of the run and of each scope, and the largest rise
  inside one entry of a scope -- never allocating while it counts. A block
  freed in another scope is credited back to the scope that allocated it; a
  table that fills, a frame too deep, another thread's call, a name with a dot
  or a guard left out of order is the run's status instead of a figure.
- Runs and profiles (`stingy4rust::run`): a profile is a plain function in a
  registry, handed a `Run` whose scopes, operations, wire meter and storage
  meters reach a `Recorder`.
- The report (`stingy4rust::report`): every figure as a dotted path, written as
  JSON by the core's own writer, keys in a fixed order.
- `stingy4rust-host`: `Stingy`, the counting `GlobalAlloc` over any allocator,
  and `HostStingy` over the system's; `ProfileMain`, a profile target's `main`,
  which runs each profile on one thread and writes the report to the file
  `STINGY4RUST_REPORT` names, exiting 2 if a run could not be measured.
- Budgets (`stingy4rust::budget`): a run's figures compared with a target's
  budget -- within, a gain, over, unbudgeted or missing -- with zero tolerance
  unless a figure carries one, and recorded as it.
- The wire, storage and operations: `WireMeter::sent`, `StorageMeter`'s
  `written`, `transaction`, `at_rest` and `on_disk`, and `Run::operation`,
  counted in the ledger's table of names -- kinds, stores and operations
  together, bounded by `NAMES` -- and reported as `wire.<kind>.*`,
  `storage.<store>.*` and `operations.<name>.*`. `docs/DECORATORS.md` shows
  how a project meters its ports with one decorator each.
- `cargo stingy4rust report|check|record`: finds the package whose `stingy`
  test target holds the profiles, runs it in release, and prints a table or
  JSON; exits 0 within budget, 1 over it, 2 unable to measure. The budget file
  is written with sorted keys, so a change reads as a diff.

## [0.0.1] - 2026-11-22

The names reserved: `stingy4rust-core` and `cargo-stingy4rust` on 2026-11-20,
`stingy4rust-host` and `stingy4rust-board` on 2026-11-22. Nothing measures yet;
the subcommand exits with an error rather than pass a gate it does not check.
