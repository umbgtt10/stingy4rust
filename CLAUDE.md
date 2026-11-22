# stingy4rust

## Meaning

`stingy4rust` is a `cargo` subcommand that holds a Rust codebase to budgets of
heap, peak heap, storage and wire bytes, measured under fixed, deterministic
load profiles the codebase writes, and checked in CI: growth fails, a saving
passes and is reported.

The repository is a workspace. `core/` publishes as `stingy4rust-core`, library
`stingy4rust`: the counting, the figures, the report, the budgets and the
comparison -- `no_std`, safe, free of every technology. `host/` publishes as
`stingy4rust-host`: the host's counting allocator, the only `unsafe` code, and
the profile target's `main`. `cli/` publishes as `cargo-stingy4rust`, library
`stingy4rust_cli`: the command line and the adapters. `validation/` runs the
built subcommand against the corpus in `fixture/`, a workspace of its own
outside this one; `xtask/` runs the gates.

`docs/DESIGN.md` is the design, decided on 2026-11-21; `docs/ADRs/` holds the
decisions and what forced them; `docs/IMPLEMENTATION-PLAN.md` is how it is
built, slice by slice, for a reader who knows nothing of it. Read the design
and the plan's sections 1 to 4 before changing anything.

## Boundary Rule

This repository is **SELF-CONTAINED**.

The LLM **SHALL NOT cross its boundaries without asking**.

That means:
- do not inspect, edit, or rely on files outside `stingy4rust/` unless the user
  explicitly asks
- do not pull assumptions from sibling repositories or crates
- do not propose cross-repository changes by default

The one standing exception: slice 0 copies its scaffolding from `../dry4rust`,
read-only, as `docs/IMPLEMENTATION-PLAN.md` section 5 lists.

## Quality Gates

### Mandatory after every change

Run:

`just stage1`
`just stage2`

If either gate is not green, the work is not complete. (Slice 0 builds them;
until it is merged, there are none.)

Both run identically on Windows, Linux and macOS, and CI runs the same two
commands -- there is no second definition of the gates to drift out of step.

Stage 1 is formatting, clippy over every target with warnings as errors, the
core's check for `thumbv7em-none-eabihf` -- it is `no_std` for microcontrollers,
and a host build proves nothing about them -- and the tests. Cargo built-ins
only. It starts with `just sync`, `cargo update` over the workspace, whose
moves are committed on their own.

Stage 2 is `cargo xtask stage2`, a crate under `xtask/` gated like any other
code, each gate a `Gate` over a `CommandRunner`. In this order:

| gate | asks |
|---|---|
| technology-free core | does `core` name only `spin`, carry `#![no_std]` and `#![forbid(unsafe_code)]`, reach no `std`; is `unsafe` confined to `host/src/stingy.rs` (and `board/src/board_stingy.rs`) |
| `cargo stern4rust` | do the house coding rules hold |
| `cargo shear` | is every declared dependency used, in the section that uses it |
| `cargo dry4rust` | did this change add duplicated code (floor 25, zero ceilings against `dry4rust-baseline.json`) |
| `cargo crap4rust` | is any function complex and untested |
| `cargo twin4rust` | does every source file have a mirrored test file |
| `cargo iceberg4rust` | is any file's private implementation risk too high |
| `cargo deny` | do the dependencies carry an advisory, a refused licence or a foreign source |
| own budget (from slice 3) | does this checkout's subcommand, built from source, find the fixture within `fixture/stingy4rust-budget.json` |

The technology-free gate runs first because its fix moves code between crates,
which changes what every gate behind it measures; stern next, because its
corrections are renames and moves; the tool's own budget last, because it
builds the fixture in release.

`cargo install just`
`cargo install --locked cargo-llvm-cov`
`cargo install --locked cargo-stern4rust`
`cargo install --locked cargo-dry4rust`
`cargo install --locked cargo-crap4rust`
`cargo install --locked cargo-twin4rust`
`cargo install --locked cargo-iceberg4rust`
`cargo install --locked cargo-shear --version 1.14.0`
`cargo install --locked cargo-deny --version 0.20.2`

### Every commit carries core's scores

Every commit message carries the grip and braintax of `core`, and their ratio,
scored as the commit stages it: `.githooks/commit-msg` runs
`cargo xtask grip-braintax` on the message. `just hooks`, once a clone, on each
machine. Never `--no-verify`.

`cargo install cargo-grip4rust`
`cargo install cargo-braintax4rust`

## Technology-free core

`core` touches no technology, so mutation testing needs nothing but its own
tests. What it needs from the machine is a trait -- `Recorder`, `Measuring` --
implemented by `host` (and, later, `board`); the formats it writes are its own
(the report's JSON, keys in a fixed order); the formats it reads are `cli`'s,
over records of its own, converted at the edge. The counting path never
allocates and never panics (ADR-CountingNeverAllocates).

## Strict Rules

### Test preservation

**NEVER remove a test without explicitly asking the user first.**

If a code change causes a test to fail, the correct response is to update the
test assertion to reflect the new correct behavior — not to delete the test. A
test documents a contract. Deleting it silently removes coverage and hides
regressions.

### Assertion integrity

**NEVER relax a test assertion without explicitly asking the user first.**

Relaxing means changing a specific assertion to a weaker one. If a code change
makes a previous assertion wrong, understand why, then update the assertion to
the new correct specific value — not to a weaker form that would pass
regardless of correctness.

### Test first

Every change starts with a test seen failing for the reason expected.

### No ignored tests, no regression files

No `#[ignore]`. `*.proptest-regressions` files are never committed; a case a
property test finds becomes a named test.

### Figures are not edited to pass

A fixture figure, a budget or a tolerance is never changed to make a gate pass.
A figure that differs is investigated; a budget is re-recorded only in the
slice that introduces its figures, or at the owner's word.

### Visibility is not an escape hatch

`tested-public-api` asks for a test, not a smaller surface. Do not drop `pub`
to `pub(crate)` to silence it.

### What only the owner does

Merging into `main`, publishing, tagging, re-recording a budget or a baseline,
changing an ADR, adding a dependency to `core`, reserving a crate name.

## User coding standards

- one object per file; a record -- public fields and derives, no behaviour --
  is a file of its own and has no test of its own
- do not use fully qualified paths; use `use` imports instead
- no re-export modules: every `use` names the module that defines the item
- unit tests are not allowed; integration tests only, under each crate's
  `tests/`, or `validation/tests/` when the subject is the whole tool -- a
  scenario, paired with its `validation/src/<scenario>_scenario.rs`
- no unnecessary comments in code
- no `&mut` input parameters; prefer return values (`&mut self` is allowed in
  traits and impl blocks)
- all dependencies injected through the single constructor, as traits where a
  test needs a fake
- only use `pub mod` in `mod.rs` and `lib.rs`
- split test files so there is one test file per source file, named
  `<source file name>_tests.rs`
- in `all_tests.rs`, reference test files one by one without `#[path = ...]`
- apply AAA (`Arrange`, `Act`, `Assert`) structure to tests, with blank-line
  separation between the sections
- use `// Arrange & Act` if there is no separate `Arrange`
- use `// Act & Assert` if there is no separate `Act`
- add the repository copyright and license header (`docs/header.txt`) to every
  Rust source file
- **test naming**: `<method>_<description>_<outcome>` where `<method>` is the
  function called in the Act section
