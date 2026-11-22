# stingy4rust

A cargo subcommand that holds a Rust codebase to **budgets**: heap, peak heap,
storage and wire bytes, measured under **fixed load profiles** and checked in
CI, so what a change costs is seen and what it saves is kept.

**In development.** Version 0.0.1 reserves the names `cargo-stingy4rust`,
`stingy4rust-core`, `stingy4rust-host` and `stingy4rust-board`; it measures
nothing yet, and the subcommand exits with an error rather than pass a gate it
does not check. The design is decided and the implementation is planned, slice
by slice (below).

## What it will do

- **Heap**: allocations, bytes requested and bytes live, counted by an
  allocator wrapper in scopes the harness opens -- one a component, one a step --
  so a figure is the code's, never the harness's.
- **Peaks**: the high-water mark of the run, of each component and of each step.
- **Storage**: bytes at rest, bytes written and transactions, counted by a
  decorator over a storage port, beside each store's size on disk.
- **Wire**: the bytes of each kind of message, and the messages an operation
  costs, counted by a decorator over a transport port.
- **Budgets**: one file at the workspace root, keyed by target and profile,
  checked with zero tolerance unless a figure is given one: a run fails on
  growth and admits a gain, and the budget is re-recorded deliberately, as a
  baseline is.
- **Reports**: a table for a person, JSON for a gate.

The load profiles are the codebase's own: deterministic scenarios from a fixed
seed, so every figure is reproducible. It is built first for a distributed
protocol that runs on hosts and on microcontrollers, where every byte counts.

A project keeps them in a test target of its own named `stingy`, with no
harness, whose `main` installs the counting allocator and hands it the
registry:

```rust
#[global_allocator]
static STINGY: HostStingy = Stingy::new(System);

static PROFILES: [Profile; 1] = [Profile { name: "idle", seed: 2026, body: idle }];

fn idle(run: &Run) {
    let _node = run.scope_numbered("node", 1);
    // the project's own system, under a fixed load
}

fn main() -> ExitCode {
    ProfileMain::new(&STINGY, &PROFILES, &ProcessEnvironment, &FileReportSink).run()
}
```

Run with `STINGY4RUST_REPORT` naming a file, it runs each profile on one
thread and writes every figure to it as JSON.

## Usage

```
cargo stingy4rust report [--package P] [--profile NAME] [--target T] [--format table|json]
cargo stingy4rust check  [--package P] [--profile NAME] [--target T] [--format table|json] [--budget PATH]
cargo stingy4rust record [--package P] [--profile NAME] [--target T] [--budget PATH]
```

- `report` runs the profiles, in release, and prints their figures; no budget
  is read.
- `check` runs them and holds every figure against the budget,
  `stingy4rust-budget.json` at the workspace root unless `--budget` names
  another; it never writes.
- `record` runs them and writes their figures as the budget of this target --
  every profile, or the one named -- keeping the tolerances of figures still
  measured, and prints what moved. Recording is deliberate: the gate never
  records on its own.

The package is the one with a test target named `stingy`, or the one
`--package` names; the target is the host's unless `--target` names one.

A profile meters its transport and its storage through one thin decorator
over each port: [`docs/DECORATORS.md`](docs/DECORATORS.md).

| exit code | means |
|---|---|
| 0 | within budget -- gains included, with a hint to record them |
| 1 | a figure over its budget, measured with none, or budgeted and not measured |
| 2 | unable to measure: no profile target, a build that failed, a run not measured, a report or a budget that cannot be read |

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
