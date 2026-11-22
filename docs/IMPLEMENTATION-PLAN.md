# stingy4rust -- the implementation plan

**Status:** ready to build, 2026-11-21. Written for a reader -- a person or an
agent -- who knows nothing of this repository, its family of tools, or the
project it was built for. Read sections 1 to 4 once; then build one slice at a
time, in order, each on its own branch.

What the tool is and why it is shaped this way: [`DESIGN.md`](DESIGN.md) (read
it first, all of it) and [`ADRs/`](ADRs/README.md). This plan says how to build
it. Where the two disagree, the design wins, and the disagreement goes to the
owner (section 13).

## 1. Orientation

- **What stingy4rust is**: a cargo subcommand, `cargo stingy4rust`, that runs
  a project's *load profiles* -- deterministic runs of the project's own code --
  under a counting global allocator, measures heap, peaks, storage and wire
  figures, and compares them with a budget file checked into the project.
  Growth fails the gate; a saving passes and is reported.
- **Who owns it**: the repository's owner, Umberto Gotti. Merging a branch,
  publishing a crate and changing a decision are the owner's acts, never yours
  (section 3.6).
- **The family**: stingy4rust is one of the owner's `*4rust` tools -- dry4rust
  (duplication), stern4rust (house coding rules), crap4rust (complexity times
  missing coverage), twin4rust (every source file has a test file), iceberg4rust
  (a file's private risk). They gate each other: each tool's own repository runs
  all the others on itself. stingy4rust copies its scaffolding from **dry4rust**,
  the closest sibling: <https://github.com/umbgtt10/dry4rust>. Clone it beside
  this repository as `../dry4rust` before slice 0; you copy from it, never edit
  it.
- **The first consumer** is the owner's distributed protocol, whose crates run
  on hosts and on microcontrollers. You do not need it until slice 5, and slice
  5 is done in that repository, at the owner's word.

## 2. Prerequisites

Install once, on the machine:

```sh
# The compiler: rust-toolchain.toml pins 1.99.1 with rustfmt, clippy and
# llvm-tools; rustup installs it on the first cargo command in this directory.
rustup show

cargo install just
cargo install --locked cargo-llvm-cov
cargo install --locked cargo-stern4rust
cargo install --locked cargo-dry4rust
cargo install --locked cargo-crap4rust
cargo install --locked cargo-twin4rust
cargo install --locked cargo-iceberg4rust
cargo install --locked cargo-shear --version 1.14.0
cargo install --locked cargo-deny --version 0.20.2
cargo install cargo-grip4rust
cargo install cargo-braintax4rust
```

Then, once a clone, after slice 0 has added the hook: `just hooks`.

`cargo stern4rust --rules` lists every house rule the stern gate applies, each
with an example that breaks it and the same example put right. Read it once.

## 3. How to work

### 3.1 The house rules

These hold for every line in the repository. A gate enforces most of them; the
rest are the owner's and are checked in review.

1. **Test first.** Write the test, run it, see it fail for the reason you
   expect, then write the code. A test never seen failing proves nothing.
2. **Never remove a test, never relax an assertion**, without the owner's word.
   If a change makes an assertion wrong, find out why, then update it to the
   new correct, specific value. `assert_eq!(x, 1)` never becomes
   `assert!(x >= 0)`.
3. **No `#[ignore]`.** A test that cannot run in a stage belongs to a package
   that stage leaves out. **`*.proptest-regressions` files are never
   committed**; a case proptest finds becomes a named unit test.
4. **Integration tests only**: tests live under `<crate>/tests/`, never in
   `src/`. One test file a source file, named `<source file>_tests.rs`, in the
   folder mirroring the source's (`core/src/ledger/block_table.rs` is tested by
   `core/tests/ledger/block_table_tests.rs`). `tests/all_tests.rs` declares
   each folder's `mod.rs`, which declares each test file one by one; never
   `#[path = ...]`.
5. **AAA**: every test has `// Arrange`, `// Act`, `// Assert`, separated by
   blank lines; `// Arrange & Act` or `// Act & Assert` where two coincide.
6. **Test names** are `<method>_<description>_<outcome>`, `<method>` being the
   function called in `// Act`: `allocated_of_three_blocks_in_a_scope_charges_it_192_bytes`.
   Test functions in a file are in alphabetical order, after the helpers.
7. **One object a file.** A *record* -- a struct of public fields and derives,
   no behaviour -- is a file of its own and gets no test of its own. An
   *object* -- a struct with behaviour -- has private fields and its own test
   file.
8. **Every `.rs` file starts with the header** in `docs/header.txt`, verbatim.
9. **`use` imports**, never fully qualified paths in code. No re-export
   modules: every `use` names the module that defines the item. Only `mod.rs`
   and `lib.rs` declare `pub mod`.
10. **No `&mut` parameters** but `&mut self`; return values instead.
11. **Dependencies injected through the one constructor** and stored in the
    struct; depend on traits, not on concrete types, wherever a test needs a
    fake. A trait for every piece of technology the core needs; its fakes in
    `tests/support/`.
12. **No unnecessary comments in code**; a comment above each type and each
    non-obvious function says what it is or does, in the repository's prose.
13. **Folders by role** once a folder nears 20 files (stern enforces the
    limit).
14. **No `unsafe`** but in `host/src/stingy.rs` (and, in slice 6,
    `board/src/board_stingy.rs`), which carry `#![allow(unsafe_code)]` under a
    crate-level `#![deny(unsafe_code)]`. The core and the subcommand
    `#![forbid(unsafe_code)]`.

### 3.2 The gates

- **`just stage1`**: format, clippy over every target with warnings as errors,
  the `no_std` check of the core, the tests. Cargo built-ins only.
- **`just stage2`**: `cargo xtask stage2` -- the technology-free core, stern,
  shear, dry, crap, twin, iceberg, deny, and from slice 3 the tool's own budget
  on the fixture (section 4.3).

**Both green after every commit.** A commit that leaves a gate red is not done.
CI runs the same two commands on Linux, Windows and macOS.

### 3.3 Branches, commits, pushes

- `git fetch` before branching, merging or pushing: the owner works from two
  machines.
- One branch a slice, from `main`: `slice-0-scaffolding`, `slice-1-ledger`, ...
- Small commits, each one coherent and green. A subject in the imperative
  saying what changed (`Count reallocations as a free and an allocation`); a
  body saying why, and anything a reader of the history would ask. If an agent
  wrote it, end with the agent's `Co-Authored-By:` trailer.
- If the owner's blocklist gate is installed on the machine
  (`~/claude-scripts/blocklist-gate/gate.py`), run it on the message before
  every commit -- `python3 ~/claude-scripts/blocklist-gate/gate.py <message file> origin/main`
  -- and commit only when it exits 0. Never print its list.
- **Never `--no-verify`.** The commit-msg hook writes the core's grip and
  braintax scores into every message; a hook that fails stops the commit, and
  the failure is fixed, not skipped.
- Push your branch when its gates are green. **Never merge into `main`, never
  publish, never tag a release** without the owner's word.

### 3.4 The figures you will assert

Many tests assert exact byte counts. They are exact because the build is
deterministic: pinned toolchain, release profile, one thread, fixed seeds, no
`HashMap` with a random state (use `BTreeMap`). Use `core::hint::black_box`
around fixture allocations, or the optimiser may remove an allocation nobody
reads.

### 3.5 When the plan does not say

Section 13. Bring options and a recommendation; do not decide what the owner
decides.

### 3.6 What only the owner does

Merging into `main`; publishing to crates.io; tagging and GitHub releases;
re-recording any budget or baseline outside the slice that introduces its
figures; changing an ADR; adding a dependency to the core; reserving a crate
name.

## 4. The repository, once built

```
stingy4rust/
├── Cargo.toml                  workspace: core, host, cli, validation, xtask; fixture excluded
├── rust-toolchain.toml         1.99.1 (already there)
├── justfile                    stage1, stage2, sync, hooks
├── stern4rust.toml             header-file = "docs/header.txt"
├── deny.toml                   the family's
├── dry4rust-baseline.json      empty: {"version":1,"groups":[]} or as dry4rust records it
├── CLAUDE.md                   the repository's rules (already there)
├── CHANGELOG.md
├── .githooks/commit-msg        runs `cargo xtask grip-braintax "$1"`
├── .github/workflows/ci.yml    stage1 and stage2 on three systems
├── core/                       stingy4rust-core, library `stingy4rust` -- no_std, safe
├── host/                       stingy4rust-host -- the host allocator, ProfileMain
├── cli/                        cargo-stingy4rust, library `stingy4rust_cli`
├── validation/                 end-to-end scenarios (publish = false)
├── xtask/                      the gates (publish = false)
├── fixture/                    a workspace of its own, outside this one
│   ├── Cargo.toml              members: measured, overflowing, threaded, unmeasured
│   ├── stingy4rust-budget.json the fixture's budget: the tool's own gate
│   ├── measured/               profiles with exactly known figures
│   ├── overflowing/            a ledger too small for its profile
│   ├── threaded/               a profile that allocates on a second thread
│   └── unmeasured/             no `stingy` target at all
└── docs/                       DESIGN, IMPLEMENTATION-PLAN, ADRs, DECORATORS (slice 4)
```

### 4.1 The core's modules

| folder | files |
|---|---|
| `core/src/ledger/` | `ledger.rs` (`Ledger`), `ledger_tables.rs` (`LedgerTables`, the state behind the lock), `block_table.rs` (`BlockTable`), `block_entry.rs` (record), `scope_table.rs` (`ScopeTable`), `scope_slot.rs` (record), `frame_stack.rs` (`FrameStack`), `frame.rs` (record), `name_table.rs` (`NameTable`), `name_slot.rs` (record), `run_status.rs` (enum), `bookkeeping.rs` (record), `max_depth.rs` (the constant) |
| `core/src/figures/` | `scope_name.rs`, `heap_figures.rs`, `scope_figures.rs`, `wire_figures.rs`, `store_figures.rs`, `operation_figures.rs`, `tally.rs` (records), `figure.rs` (record: path, value), `flat_figures.rs` (`FlatFigures`: a tally as paths and values) |
| `core/src/run/` | `recorder.rs` (trait), `measuring.rs` (trait), `profile.rs` (record), `run.rs` (`Run`), `scope_guard.rs`, `operation_guard.rs`, `wire_meter.rs`, `storage_meter.rs` |
| `core/src/report/` | `report.rs`, `profile_report.rs` (records), `json_writer.rs` (`JsonWriter`), `report_json.rs` (`ReportJson`) |
| `core/src/budget/` | `budget.rs`, `profile_budget.rs`, `figure_verdict.rs`, `check_outcome.rs` (records), `tolerance.rs` (`Tolerance`), `verdict.rs` (enum), `comparison.rs` (`Comparison`), `recording.rs` (`Recording`) |

A module path is its folder and file: `stingy4rust::ledger::ledger::Ledger`,
`stingy4rust::run::run::Run`.

### 4.2 The fixture's exact figures

`fixture/measured/tests/stingy.rs` (slice 2, completed in slice 4) holds these
profiles, every allocation wrapped in `black_box`, seed 2026 each:

| profile | body | figures it must report (x86_64, toolchain 1.99.1) |
|---|---|---|
| `three-blocks` | in scope `worker`: three `Box<[u8; 64]>`, dropped before the guard | `scopes.worker`: allocations 3, reallocations 0, bytes_allocated 192, bytes_freed 192, live_at_end 0, peak 192, largest_entry_rise 192, entries 1; `heap.allocations` 3; `peaks.run` 192 |
| `freed-elsewhere` | a `Box<[u8; 100]>` made in scope `producer`, moved out, dropped in scope `consumer` | `scopes.producer`: allocations 1, bytes_allocated 100, bytes_freed 100, live_at_end 0; `scopes.consumer`: allocations 0, bytes_freed 0, entries 1 |
| `nested` | scope `outer` keeps a `Box<[u8; 10]>` while scope `inner` keeps a `Box<[u8; 20]>` | `scopes.outer`: allocations 1, bytes_allocated 10; `scopes.inner`: allocations 1, bytes_allocated 20; `peaks.run` 30 |
| `growing` | in scope `grower`, a `Vec<u8>` pushed 0 to 99 | `scopes.grower`: allocations 1, reallocations 4, bytes_allocated 248 (8+16+32+64+128), bytes_freed 248, peak 128 |
| `outside` | a `Box<[u8; 32]>` made and dropped with no scope open | `scopes.(outside scopes)`: allocations 1, bytes_allocated 32, bytes_freed 32 |
| `wire-and-storage` (slice 4) | operation `exchange` twice, each sending `ping` of 10 bytes and `pong` of 100; store `journal` written 50 bytes three times, two transactions, 150 at rest, 4096 on disk | `wire.ping`: messages 2, bytes 20; `wire.pong`: messages 2, bytes 200; `operations.exchange`: count 2, messages 4, bytes 220; `storage.journal`: bytes_written 150, writes 3, transactions 2, bytes_at_rest 150, bytes_on_disk 4096 |

If a figure differs on first run, do not change the expectation to match:
find out why (an allocation not wrapped in `black_box`, a growth strategy that
differs), and if the toolchain genuinely does otherwise, bring it to the owner.

### 4.3 The tool gates itself

From slice 3, the last gate of `cargo xtask stage2` runs this checkout's
subcommand, built from source, on the fixture:
`cargo run --release --bin cargo-stingy4rust -- stingy4rust check --package measured`
with `fixture/` as the working directory, and fails unless it exits 0.

## 5. Slice 0 -- the scaffolding

**Goal**: the workspace, gates, hook and CI of the family, green, with crates
that do nothing yet.

**Copy from `../dry4rust`, then adapt:**

| file | adapt |
|---|---|
| `justfile` | rename every `dry4rust`; keep `sync`, `stage1`, `stage2`, `hooks`; in `stage1` add `cargo check -p stingy4rust-core --no-default-features --target thumbv7em-none-eabihf` once the core has code (install the target with `rustup target add thumbv7em-none-eabihf`; add it to `rust-toolchain.toml`'s `targets`) |
| `xtask/` | keep the `Gate` trait, `CommandRunner`, `Stage2`, the stern, shear, crap, twin, iceberg and deny gates, `sync` and `grip-braintax`; replace the dry self-gate with one that runs the **installed** `cargo dry4rust` against `dry4rust-baseline.json` at floor 25, as dry4rust's own gate does from source; adapt the technology-free gate to: `core`'s manifest names only `spin`; `core/src/lib.rs` carries `#![no_std]` and `#![forbid(unsafe_code)]`; no file under `core/` reaches `std`; and **`unsafe` appears nowhere but `host/src/stingy.rs`** (and later `board/src/board_stingy.rs`); the grip/braintax score is the core's, as in dry4rust |
| `.githooks/commit-msg` | as is |
| `.github/workflows/ci.yml` | as is, the tool list of section 2, `cargo-dry4rust` installed rather than built |
| `deny.toml` | as is |
| `stern4rust.toml` | `header-file = "docs/header.txt"`; nothing skipped |
| `dry4rust-baseline.json` | record it once the crates exist: `cargo dry4rust --path . --min-nodes 25 --baseline "$PWD/dry4rust-baseline.json" baseline` -- it should hold no group |
| `CHANGELOG.md` | dry4rust's preamble, an `Unreleased` section |

**Already there**: `host/`, the placeholder that reserved `stingy4rust-host`
(`lib.rs` with `#![deny(unsafe_code)]`, nothing else yet), in the root
workspace.

**Create**: `validation/` (`publish = false`), `fixture/Cargo.toml` (a
workspace of its own, no members yet), and add `validation` to the root
workspace, `fixture` to `exclude`. Every new `.rs` file carries the header.
Each crate takes the family's `[lints]` tables as dry4rust's do.

**Tests first**: the xtask's tests come with it from dry4rust; adapt their
expectations to the new gate list and the unsafe-confinement rule, and add one:
`check_of_a_core_using_unsafe_fails_naming_the_file` (or the closest name the
xtask's structure gives).

**Acceptance**: `just stage1` and `just stage2` green on Linux; CI green on the
three systems once pushed; `just hooks` adds the scores to a commit message.

**Docs**: the README's "In development" stays; `CLAUDE.md` gains nothing (it
already describes this layout).

## 6. Slice 1 -- the ledger

**Goal**: every heap figure of section 2.1 of the design, counted by a ledger
that never allocates, tested without any allocator: the tests call the
ledger's methods with made-up addresses.

**Files**: the `core/src/ledger/` and `core/src/figures/` rows of section 4.1.
Add `spin` to the core: `spin = { version = "0.12", default-features = false, features = ["spin_mutex"] }`
(0.12.3 was the newest on 2026-11-21, its `spin_mutex` feature `no_std` and
free of allocation; it is the core's only dependency,
ADR-TechnologyFreeSafeCore).

**API** (`core/src/ledger/ledger.rs`):

```rust
pub struct Ledger<const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> {
    tables: spin::Mutex<LedgerTables<SCOPES, BLOCKS, NAMES>>,
}

impl<const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> Ledger<SCOPES, BLOCKS, NAMES> {
    pub const fn new() -> Self;
    pub fn allocated(&self, address: usize, size: usize, thread: usize);
    pub fn freed(&self, address: usize, size: usize, thread: usize);
    pub fn reallocated(&self, old_address: usize, old_size: usize,
                       new_address: usize, new_size: usize, thread: usize);
    pub fn begin(&self, thread: usize);
    pub fn end(&self) -> Tally;
    pub fn bookkeeping(&self) -> Bookkeeping;
}
```

`enter` and `leave` come with the `Recorder` trait in slice 2; in this slice
`Ledger` has them as plain methods: `pub fn enter(&self, scope: ScopeName) -> usize`
and `pub fn leave(&self, depth: usize)`.

**Rules**:

- `BLOCKS` must be a power of two: `const { assert!(BLOCKS.is_power_of_two()) }`
  in `new`.
- The block table hashes `address >> 4` with a 64-bit multiplicative hash
  (`(address as u64 >> 4).wrapping_mul(0x9E37_79B9_7F4A_7C15) >> (64 - BLOCKS.trailing_zeros())`),
  probes linearly, deletes by backward shift.
- Every counter is a `u64` and every arithmetic step saturates: nothing in the
  counting path may panic.
- `begin` clears every table and records the thread; before `begin` and after
  `end`, every counting call returns at once.
- Allocations with no scope open are charged to slot 0, `(outside scopes)`.
- A free or a reallocation of an address the table does not hold counts one
  `untracked_frees` and, for a reallocation, charges the new block as an
  allocation.
- `end` stops counting first, then builds the `Tally` (which allocates).
- `Tally.scopes` lists `(outside scopes)` first, then scopes in the order first
  entered.

**Tests to write first** (`core/tests/ledger/ledger_tests.rs`, using
`Ledger<4, 8, 4>` and the thread mark `1`; the other files' tests follow the
same pattern):

| test | asserts |
|---|---|
| `allocated_before_begin_is_not_counted` | a tally of a run begun after the allocation shows 0 allocations |
| `allocated_from_another_thread_while_a_scope_is_open_reports_foreign_thread` | status `ForeignThread`; the allocation not counted |
| `allocated_of_three_blocks_in_a_scope_charges_it_192_bytes` | the scope's allocations 3, bytes_allocated 192, live_at_end 192, peak 192 |
| `allocated_outside_any_scope_charges_the_outside_scopes_slot` | `(outside scopes)` allocations 1 |
| `allocated_past_the_block_capacity_reports_blocks_exhausted` | nine live blocks in `Ledger<4, 8, 4>`: status `BlocksExhausted`, eight counted |
| `end_lists_the_outside_scopes_slot_first_then_scopes_as_first_entered` | the tally's scope names in order |
| `end_of_a_scope_entry_that_rose_and_fell_reports_its_largest_rise` | allocate 100, free it, allocate 30, free it, all in one entry: `largest_entry_rise` 100, `peak` 100, `live_at_end` 0 |
| `end_reports_the_run_peak_across_scopes` | 10 live in one scope while 20 live in another: `peak` 30 |
| `enter_of_a_name_with_a_dot_reports_name_with_dot` | status `NameWithDot` |
| `enter_past_the_depth_reports_depth_exhausted` | 65 nested scopes: status `DepthExhausted` |
| `enter_past_the_scope_capacity_reports_scopes_exhausted` | a fourth distinct name in `Ledger<4, ...>`, whose slot 0 is `(outside scopes)`: status `ScopesExhausted` |
| `enter_of_a_name_entered_before_counts_one_more_entry` | `entries` 2 |
| `freed_of_a_block_allocated_before_begin_counts_an_untracked_free` | `untracked_frees` 1, every figure 0 |
| `freed_of_a_block_in_another_scope_credits_the_scope_that_allocated_it` | the allocating scope's bytes_freed 100, live 0; the freeing scope's 0 |
| `leave_out_of_order_reports_scope_left_out_of_order` | status `ScopeLeftOutOfOrder` |
| `reallocated_counts_a_reallocation_freeing_the_old_size_and_charging_the_new` | allocations 1, reallocations 1, bytes_allocated 8+16, bytes_freed 8, live 16 |
| `reallocated_in_another_scope_charges_the_new_block_to_the_scope_open` | the old size credited to the first scope, the new charged to the second |

`block_table_tests.rs`: `insert_then_remove_leaves_the_table_empty`;
`remove_of_a_block_in_a_probe_chain_keeps_the_others_found` (three addresses
hashing to one slot); `insert_into_a_full_table_reports_it_full`.
`frame_stack_tests.rs`, `scope_table_tests.rs` and `name_table_tests.rs` test
their own objects the same way.

**Acceptance**: every test above green; the core builds for
`thumbv7em-none-eabihf`; stage 2 green (crap will ask for these tests; iceberg
may ask a big file to split -- split it by role, never raise a threshold).

**Docs**: `CHANGELOG.md` `Unreleased`: the ledger.

## 7. Slice 2 -- runs, profiles, the host allocator, the report

**Goal**: a project's profile target runs its profiles under the counting
allocator and writes the report of design section 7.2; the fixture's first
five profiles report exactly the figures of section 4.2.

**Files**: `core/src/run/` and `core/src/report/` (section 4.1);
`core/src/figures/figure.rs` and `flat_figures.rs`; `host/src/`: `stingy.rs`
(`Stingy`), `host_stingy.rs` (the type `HostStingy`), `thread_mark.rs`
(`ThreadMark`), `environment.rs` (trait), `report_sink.rs` (trait),
`process_environment.rs`, `file_report_sink.rs`, `profile_main.rs`
(`ProfileMain`); `fixture/measured/` (package `measured`, `[[test]] name =
"stingy"`, `harness = false`, dev-dependencies on `../../core` and `../../host`
by path); `validation/src/fixture_run.rs` and its scenario.

**API**:

```rust
// core/src/run/recorder.rs -- the design's section 5.1, verbatim.
// core/src/run/measuring.rs
pub trait Measuring: Recorder {
    fn begin(&self);
    fn end(&self) -> Tally;
}
// core/src/run/run.rs -- the design's section 5.2.
// core/src/figures/flat_figures.rs
pub struct FlatFigures;
impl FlatFigures { pub fn of(tally: &Tally) -> Vec<Figure>; }     // design 7.1's order
// core/src/report/report_json.rs
pub struct ReportJson;
impl ReportJson { pub fn write(report: &Report) -> String; }       // design 7.2, keys in a fixed order
// host/src/stingy.rs
pub struct Stingy<A, const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> {
    inner: A,
    ledger: Ledger<SCOPES, BLOCKS, NAMES>,
}
impl<A, ...> Stingy<A, ...> { pub const fn new(inner: A) -> Self; }
unsafe impl<A: GlobalAlloc, ...> GlobalAlloc for Stingy<A, ...> { /* alloc, alloc_zeroed, dealloc, realloc */ }
impl<A: GlobalAlloc, ...> Recorder for Stingy<A, ...> { /* to the ledger */ }
impl<A: GlobalAlloc, ...> Measuring for Stingy<A, ...> { /* begin with ThreadMark::current() */ }
// host/src/host_stingy.rs
pub type HostStingy = Stingy<System, 128, 262_144, 256>;
// host/src/thread_mark.rs
pub struct ThreadMark;
impl ThreadMark { pub fn current() -> usize; }   // address of a const-initialised thread_local!
// host/src/environment.rs, report_sink.rs
pub trait Environment { fn variable(&self, name: &str) -> Option<String>; }
pub trait ReportSink { fn write(&self, path: &str, text: &str) -> Result<(), SinkFailure>; }
// host/src/profile_main.rs
impl ProfileMain {
    pub fn new(measuring: &'static dyn Measuring, profiles: &'static [Profile],
               environment: &'static dyn Environment, sink: &'static dyn ReportSink) -> Self;
    pub fn run(&self) -> ExitCode;
}
```

**Rules**:

- `Stingy`'s `GlobalAlloc` calls the inner allocator first and records only a
  non-null result; `alloc_zeroed` counts as an allocation; `realloc` is the
  inner `realloc`, recorded as one reallocation.
- `ProfileMain::run`: no `STINGY4RUST_REPORT` is an error printed to standard
  error, exit 2, nothing run; an unknown `STINGY4RUST_PROFILE` likewise. Each
  profile: `begin`, `body(&run)`, `end`; the report carries every profile run,
  even after one fails; exit 0 only if every status is `measured`.
- The report's JSON escapes `"` and `\` and control characters in names, and
  writes figures as integers.

**Tests to write first**:

| file | tests |
|---|---|
| `core/tests/run/run_tests.rs` (a fake `Recorder` in `core/tests/support/recording_recorder.rs`) | `scope_of_a_run_enters_and_its_guard_leaves_at_the_depth_entered`; `scope_numbered_names_the_scope_with_its_number`; `operation_of_a_run_begins_and_its_guard_ends_it`; `wire_of_a_run_records_through_the_run_s_recorder`; `storage_of_a_run_names_its_store` |
| `core/tests/figures/flat_figures_tests.rs` | `of_a_tally_lists_heap_peaks_then_each_scope_s_figures_in_order`; `of_a_numbered_scope_writes_its_number_in_the_path` |
| `core/tests/report/report_json_tests.rs` | `write_of_a_report_of_one_profile_is_the_documented_json` (an exact string); `write_of_a_name_with_a_quote_escapes_it`; `write_of_a_failed_run_writes_its_status_in_kebab_case` |
| `host/tests/profile_main_tests.rs` (fake `Measuring`, `Environment`, `ReportSink`) | `run_of_two_profiles_writes_both_to_the_path_named`; `run_with_a_profile_named_runs_only_it`; `run_with_no_report_path_exits_2_and_runs_nothing`; `run_of_a_profile_whose_status_is_not_measured_exits_2_and_still_writes` |
| `host/tests/thread_mark_tests.rs` | `current_on_one_thread_is_the_same_twice`; `current_on_another_thread_differs` |
| `validation/tests/fixture_run_scenario_tests.rs` | for each profile of section 4.2 but the last, one test, e.g. `run_of_three_blocks_reports_192_bytes_in_the_worker_scope`, running `cargo test --release --test stingy` in `fixture/measured` with `STINGY4RUST_REPORT` set, reading the report with `serde_json` (a validation dependency), asserting every figure listed |

**Acceptance**: the fixture's five profiles report the figures of section 4.2
exactly, twice in a row with identical reports (assert it in one more scenario:
`run_twice_writes_the_same_report`); stage 1 and stage 2 green.

**Docs**: `CHANGELOG.md`; the README's "What it will do" gains the profile
target's few lines.

## 8. Slice 3 -- the subcommand: report, check, record

**Goal**: `cargo stingy4rust report|check|record` as design section 7 states,
with the budget file, tolerances, the comparison, both outputs and the exit
codes; the tool's own gate on the fixture.

**Files**: `core/src/budget/` (section 4.1); `cli/src/`:
`command_line/` (`arguments.rs` with clap's derive -- the first argument cargo
passes is `stingy4rust`, which the parser accepts and drops, as dry4rust's
does; `command.rs`, `output_format.rs`), `ports/` (`command_runner.rs`,
`file_store.rs` -- traits), `system/` (`system_command_runner.rs`,
`disk_file_store.rs`), `discovery/` (`target_finder.rs` -- reads
`cargo metadata --format-version 1 --no-deps`; `host_triple.rs` -- reads
`rustc -vV`'s `host:` line; `profile_target.rs`, record), `measure/`
(`profile_runner.rs` -- runs `cargo test --release --package P --test stingy`
with the environment of design section 6, reads the report), `formats/`
(`report_file.rs` and `budget_file.rs` -- serde records and their conversion
into the core's; the budget written with sorted keys and two-space
indentation; `check_json.rs`; `table.rs`), `entry_point.rs` (`EntryPoint`:
takes the ports, returns an `Outcome { stdout, stderr, exit_code }`, the only
place that knows exit codes), `main.rs` (builds the system adapters, prints the
outcome, exits). `fixture/overflowing`, `fixture/threaded`,
`fixture/unmeasured`; `fixture/stingy4rust-budget.json`, recorded by the tool
itself once `record` works, and reviewed by hand against section 4.2.

**API** (core):

```rust
pub struct Tolerance { absolute: Option<u64>, relative: Option<f64> }   // object
impl Tolerance { pub fn new(absolute: Option<u64>, relative: Option<f64>) -> Self;
                 pub fn allowed(&self, budget: u64) -> u64; }
pub enum Verdict { Within, Gain, Over, Unbudgeted, Missing }
pub struct Comparison;
impl Comparison {
    pub fn of(budget: &Budget, report: &Report, target: &str, profile: Option<&str>) -> CheckOutcome;
}
pub struct Recording;
impl Recording {
    pub fn of(budget: &Budget, report: &Report, target: &str, profile: Option<&str>) -> Budget;
}
```

**Rules**: design sections 7.3 to 7.5, exactly. In particular: a gain passes;
`check` never writes; `record` keeps the tolerances of figures still measured
and drops the others; any run status other than `measured`, any build failure,
any unparsable report or budget, and a budget of an unknown `version` exit 2.

**Tests to write first**:

| file | tests |
|---|---|
| `core/tests/budget/tolerance_tests.rs` | `allowed_with_no_tolerance_is_the_budget`; `allowed_with_an_absolute_tolerance_adds_it`; `allowed_with_a_relative_tolerance_rounds_up`; `allowed_with_both_takes_the_larger` |
| `core/tests/budget/comparison_tests.rs` | `of_a_figure_equal_to_its_budget_is_within`; `of_a_figure_below_its_budget_is_a_gain_and_passes`; `of_a_figure_above_its_budget_is_over`; `of_a_figure_above_within_its_tolerance_is_within`; `of_a_figure_with_no_budget_is_unbudgeted`; `of_a_budgeted_figure_not_measured_is_missing`; `of_a_profile_with_no_budget_is_unbudgeted`; `of_a_budgeted_profile_not_run_is_missing`; `of_a_target_with_no_budget_is_unbudgeted`; `of_one_profile_named_compares_it_alone` |
| `core/tests/budget/recording_tests.rs` | `of_a_report_writes_its_figures_as_the_target_s_budget`; `of_a_report_keeps_the_tolerances_of_figures_still_measured`; `of_a_report_drops_the_tolerances_of_figures_gone`; `of_one_profile_named_replaces_it_alone`; `of_a_report_leaves_other_targets_as_they_were` |
| `cli/tests/discovery/target_finder_tests.rs` (fake runner returning metadata JSON) | `find_with_one_package_holding_a_stingy_test_is_it`; `find_with_none_fails_naming_the_target`; `find_with_two_and_no_package_named_fails_listing_them`; `find_with_a_package_named_is_it` |
| `cli/tests/discovery/host_triple_tests.rs` | `read_of_rustc_s_version_is_its_host_line` |
| `cli/tests/measure/profile_runner_tests.rs` | `measure_runs_cargo_test_in_release_with_the_report_path`; `measure_with_a_profile_named_passes_it`; `measure_of_a_build_that_fails_is_unable` |
| `cli/tests/formats/budget_file_tests.rs` | `write_of_a_budget_sorts_its_keys`; `read_of_what_write_wrote_is_the_same_budget`; `read_of_an_unknown_version_fails` |
| `cli/tests/formats/table_tests.rs`, `check_json_tests.rs` | an exact rendering of a check with one figure in each verdict |
| `cli/tests/entry_point_tests.rs` (fakes) | `run_of_check_within_budget_exits_0`; `run_of_check_over_budget_exits_1`; `run_of_check_with_an_unbudgeted_figure_exits_1`; `run_of_check_whose_run_failed_exits_2`; `run_of_record_writes_the_budget_and_exits_0`; `run_of_report_reads_no_budget` |
| `validation/tests/` | `check_scenario_tests.rs`: within (0), a budget lowered by one (1, "over"), a figure removed from the budget (1, "unbudgeted"), a budget raised by one (0, "gain"); `record_scenario_tests.rs`: record then check exits 0; `unable_scenario_tests.rs`: `overflowing` (2, "blocks-exhausted"), `threaded` (2, "foreign-thread"), `unmeasured` (2), a corrupt budget (2); `report_scenario_tests.rs`: report prints every figure and exits 0 -- each scenario copying `fixture/stingy4rust-budget.json` to a temporary directory and pointing `--budget` at the copy |

**Acceptance**: every scenario green; the xtask's last gate -- the subcommand
from source on the fixture -- green; `cargo install --path cli` then
`cargo stingy4rust check --package measured` in `fixture/` exits 0.

**Docs**: the README's usage section (the three commands, the exit codes);
`CHANGELOG.md`.

## 9. Slice 4 -- meters and operations

**Goal**: wire and storage figures and operations, as design sections 5.3 and
5.4; the fixture's sixth profile; the decorators' pattern written down.

**Files**: `core/src/run/wire_meter.rs`, `storage_meter.rs`,
`operation_guard.rs` complete; the ledger's `NameTable` used for kinds, stores
and operations; `FlatFigures` and the JSON writer extended;
`docs/DECORATORS.md` -- how a project meters its transport and its storage
with one decorator a port, with a complete example over a small `Transport`
and `Storage` trait of the doc's own.

**Rules**: `sent` charges the current operation's messages and bytes as well
as the kind's; operations do not nest (design 5.3); `at_rest` and `on_disk`
keep the last value given; `bytes_on_disk` appears in the figures only for a
store that reported it.

**Tests first**: `core/tests/ledger/name_table_tests.rs`
(`intern_past_the_capacity_reports_names_exhausted`,
`intern_of_a_name_met_before_is_its_slot`); in `ledger_tests.rs`,
`sent_while_an_operation_is_open_charges_it`, `sent_with_no_operation_open_charges_the_kind_alone`,
`written_counts_bytes_and_writes_of_the_store`, `transaction_counts_one`,
`at_rest_keeps_the_last_value`; in `wire_meter_tests.rs` and
`storage_meter_tests.rs`, each method through a fake recorder; in validation,
`run_of_wire_and_storage_reports_the_documented_figures`.

**Acceptance**: the sixth profile's figures exact; its figures recorded into
`fixture/stingy4rust-budget.json` in this slice (the slice introduces them --
the one recording a slice may do itself, reviewed against section 4.2 by
hand); all gates green.

**Docs**: `docs/DECORATORS.md`; README; `CHANGELOG.md`. Then propose the
first release (section 12) to the owner.

## 10. Slice 5 -- the first consumer (in the consumer's repository)

**Not built here.** At the owner's word, and after the first release, in the
consumer's own repository, following that repository's own `CLAUDE.md`:

- a `stingy` test target over its deterministic simulated cluster, one
  scope a node's step, one operation a client command;
- decorators over its transport and storage ports (`docs/DECORATORS.md`);
- the profiles its roadmap names: idle; 100 and 500 commands a second; a long
  log; a crashed follower; a snapshot installed; a change of membership; a
  partition healed;
- its budget recorded *before* the fixes the figures are to measure, and a
  stage 2 gate decoding exit 1 as over budget and anything else as unable.

What stingy4rust owes that slice: whatever the consumer finds missing comes
back here as an issue for the owner, not as a change made from there.

## 11. Slice 6 -- microcontrollers

**Goal**: the same profiles on a board and on QEMU, with budgets per target.

**Settled**: a crate `stingy4rust-board` (`no_std`), `BoardStingy<A: GlobalAlloc, SCOPES, NAMES>`
keeping each block's scope in a header it reserves in front of the block
(design section 9, ADR-FreedElsewhereIsCreditedExactly) -- the header's cost
reported in the bookkeeping; the report written as the host's JSON over
semihosting first; the subcommand's `--target` passed through to cargo, with
the board's runner configured in the consumer's `.cargo/config.toml`.

**To settle with the owner at the slice's start**, with a spike's findings in
hand: the header's size and alignment rule; how a `no_main` test target names
its entry and its panic handler; which board and which QEMU machine first;
whether defmt replaces semihosting on real boards. Do not start the code before
these answers.

## 12. Releases

Only at the owner's word. When asked:

1. Reserve any new crate name first (`stingy4rust-board`;
   `stingy4rust-host` is reserved, 0.0.1) -- the owner reserves names.
2. Bump `[workspace.package] version` once, for every published crate; move
   `CHANGELOG.md`'s `Unreleased` under the version, dated.
3. `cargo package --workspace`, which builds every crate as it would be
   uploaded and uploads nothing; then, at the owner's word,
   `cargo publish --workspace` (it orders the crates and skips the
   `publish = false` ones); an annotated tag `vX.Y.Z`; a GitHub release whose
   notes are the CHANGELOG's section.
4. The README's "In development" goes only when the owner says the tool is
   ready to recommend.

The first release is **0.1.0 after slice 4**: the host tool complete
(confirmed by the owner, 2026-11-22).

## 13. How to ask

Bring the owner a short note -- what you found, two or three options, the one
you recommend and why -- instead of deciding, whenever:

- a test would have to be removed or an assertion relaxed;
- a figure in section 4.2 differs from what the fixture reports, after you
  ruled out your own code;
- an ADR or the design would have to change, or the plan disagrees with the
  design;
- the core would need a dependency other than `spin`, or `unsafe` would be
  needed outside the two allowed files;
- a tolerance, a baseline or a budget would have to be re-recorded outside
  the slice that introduces its figures;
- a gate asks for a threshold or a rule to be relaxed rather than the code
  reshaped;
- anything outside this repository would have to change, or a crate name be
  reserved;
- merging, publishing, tagging.

## 14. Glossary

- **Profile**: a named, seeded, deterministic run of a project's code
  (`Profile { name, seed, body }`).
- **Scope**: a named region of a run, opened by a guard, to which
  allocations are charged.
- **Entry**: one opening of a scope.
- **Figure**: one measured value, named by its path (`scopes.node 3.peak`).
- **Budget**: the figures a profile was last recorded at, per target.
- **Tally**: what the ledger counted in one run.
- **Report**: every profile's figures, as the profile target writes them.
- **Recorder / Measuring**: the traits through which runs and the profile
  target reach the counting allocator.
- **Target triple**: the platform a budget belongs to,
  e.g. `x86_64-unknown-linux-gnu`.
