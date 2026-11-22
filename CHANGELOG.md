# Changelog

All notable changes to `cargo-stingy4rust` and its crates are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [Unreleased]

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

## [0.0.1] - 2026-11-22

The names reserved: `stingy4rust-core` and `cargo-stingy4rust` on 2026-11-20,
`stingy4rust-host` and `stingy4rust-board` on 2026-11-22. Nothing measures yet;
the subcommand exits with an error rather than pass a gate it does not check.
