# stingy4rust

A cargo subcommand that holds a Rust codebase to **budgets**: heap, peak heap,
storage and wire bytes, measured under **fixed load profiles** and checked in
CI, so what a change costs is seen and what it saves is kept.

**In development.** Version 0.0.1 reserves the names `cargo-stingy4rust` and
`stingy4rust-core`; it measures nothing yet, and the subcommand exits with an
error rather than pass a gate it does not check. The design is decided and the
implementation is planned, slice by slice (below).

## What it will do

- **Heap**: allocations, bytes requested and bytes live, counted by an
  allocator wrapper in scopes the harness opens -- one a component, one a step --
  so a figure is the code's, never the harness's.
- **Peaks**: the high-water mark of the run, of each component and of each step.
- **Storage**: bytes at rest, bytes written and transactions, counted by a
  decorator over a storage port, beside each store's size on disk.
- **Wire**: the bytes of each kind of message, and the messages an operation
  costs, counted by a decorator over a transport port.
- **Budgets**: one file a load profile, checked with a tolerance: a run fails on
  growth and admits a gain, and the budget is re-recorded deliberately, as a
  baseline is.
- **Reports**: a table for a person, JSON for a gate.

The load profiles are the codebase's own: deterministic scenarios from a fixed
seed, so every figure is reproducible. It is built first for a distributed
protocol that runs on hosts and on microcontrollers, where every byte counts.

## Design

- [`docs/DESIGN.md`](docs/DESIGN.md): what it measures and exactly how, the
  crates, the counting allocator and its scopes, runs, profiles and meters,
  how the subcommand runs a project's profiles, the report, budgets and the
  gate, determinism, microcontrollers -- decided on 2026-11-21.
- [`docs/IMPLEMENTATION-PLAN.md`](docs/IMPLEMENTATION-PLAN.md): how it is
  built, slice by slice, for a reader who knows nothing of it -- the house
  rules, the gates, each slice's files, API, tests and acceptance.
- [`docs/ADRs/`](docs/ADRs/README.md): each decision, and what forced it.
- [`CLAUDE.md`](CLAUDE.md): the repository's rules.

## Licence

MIT -- see [`LICENSE`](LICENSE).
