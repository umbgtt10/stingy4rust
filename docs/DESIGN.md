# stingy4rust -- the design

**Status:** decided, 2026-11-21. The owner settled the five questions the
proposal of 2026-11-20 left open (section 13), and on 2026-11-22 confirmed the
calls the design made beyond them (section 14); everything else below is the
design as it is to be built. How it is built, slice by slice, is
[`IMPLEMENTATION-PLAN.md`](IMPLEMENTATION-PLAN.md); why each load-bearing
choice was made is in [`ADRs/`](ADRs/README.md).

## 1. The problem

A codebase that runs on small machines has to know what it costs: how many
times it allocates, how much it keeps, how high its heap climbs, how many
bytes it stores and how many it sends. Read once, by hand, those figures are
true for a day. stingy4rust makes them a gate: the same loads, run the same
way on every change, measured the same way, and held to a budget, so a change
that costs more is seen before it is merged, and a change that saves is kept.

It is protocol-agnostic. Its first consumer is a distributed protocol whose
crates run both on hosts and on microcontrollers; the next ones are other
protocols built the same way. Nothing in it knows what a protocol, a message
or a log entry is.

## 2. What it measures

Four families of figures, each counted under a **load profile** (section 6)
and attributed to **scopes** (section 4):

| family | figures |
|---|---|
| **heap** | allocations; reallocations; bytes allocated; bytes freed; bytes live at the end |
| **peaks** | the high-water mark of bytes live -- of the run, of each scope, and the largest rise inside one entry of a scope |
| **storage** | per store: bytes written; writes; transactions; bytes at rest at the end; bytes on disk, where a store has a disk |
| **wire** | per kind of message: messages sent and bytes sent; per operation: how many, and the messages and bytes sent while it was open |

**Not time** (ADR-TimeIsNotMeasured). Durations depend on the machine and its
load; counts, bytes and peaks do not, on a deterministic run with a pinned
toolchain (section 9). Timing belongs to a benchmark, beside it.

**Bytes requested, not bytes used** (ADR-BytesRequestedNotBytesUsed). The heap
figures count `Layout::size()`. What an allocator adds on top -- headers,
alignment, fragmentation -- differs between allocators, so it is reported apart,
by the backend that knows its allocator, and never budgeted.

### 2.1 Exact definitions

- **allocation**: one `alloc` or `alloc_zeroed` call that returned a block.
- **reallocation**: one `realloc` call that returned a block. For bytes it
  counts as the old block freed and a new block of the new size allocated; for
  counts it is a reallocation, not an allocation and a free.
- **bytes allocated**: the sum of the sizes of the blocks handed out --
  `layout.size()` for an allocation, `new_size` for a reallocation.
- **bytes freed**: the sum of the sizes of the blocks given back -- `layout.size()`
  for a `dealloc`, the old size for a reallocation.
- **bytes live**: bytes allocated minus bytes freed, at any moment.
- **peak of the run**: the highest bytes live of the whole run, every scope and
  the outside-scopes bucket together.
- **peak of a scope**: the highest bytes live charged to that scope.
- **largest entry rise of a scope**: over every entry of the scope, the highest
  bytes live charged to it during that entry minus the bytes live charged to it
  when the entry began. A step that clones a snapshot six times and frees every
  copy before it ends shows here, though its live bytes end where they began.
- **entries of a scope**: how many times it was entered.

A run's allocations outside any scope are charged to a scope of their own,
named `(outside scopes)`, reported and budgeted like any other: the harness's
own cost is visible, and kept apart from the code's.

## 3. The crates

| crate (package) | library | holds | `no_std` | `unsafe` |
|---|---|---|---|---|
| `stingy4rust-core` | `stingy4rust` | the ledger that counts, scopes, peaks; `Run`, `Profile`, the guards, `WireMeter`, `StorageMeter`; the figures, the report and its JSON writer; budgets, tolerances and the comparison | yes, with `alloc` | forbidden |
| `stingy4rust-host` | `stingy4rust_host` | the host's counting allocator (a `GlobalAlloc` over `System` and the ledger); the thread mark; `ProfileMain`, the profile target's `main` | no | in its allocator file alone |
| `cargo-stingy4rust` | `stingy4rust_cli` | the subcommand: finding and running the profile target, reading the report, reading and writing the budget, comparing, printing, exit codes | no | forbidden |
| `stingy4rust-board` (slice 6) | `stingy4rust_board` | a board's counting allocator over its own `GlobalAlloc`, and the report over the board's channel | yes | in its allocator file alone |

And, not published:

- `validation/`: the end-to-end scenarios, which run the built subcommand
  against the fixture.
- `fixture/`: a consumer crate outside the workspace whose profiles allocate,
  send and store exactly known amounts, with its own budget -- the tool's
  corpus, and the budget the tool gates itself with (section 11).
- `xtask/`: the gates, as a crate.

**Why three crates where the proposal had two** (ADR-TechnologyFreeSafeCore):
a `GlobalAlloc` is `unsafe` to implement, and the family's core forbids
`unsafe` and every technology. The counting is all in the core, safe and
testable without a process; the host crate is the thin `unsafe` shim that hands
each allocator call to it, plus the `std` a profile target needs to read its
environment and write its report. The core is technology-free, so it can be
mutation-tested on its own, as the family's cores are.

The core may depend on one crate: `spin`, for a lock that needs no `std` and
no allocation (section 4.3). Every published crate shares one version, from
`[workspace.package]`.

## 4. The heap: a ledger that counts, by scope

### 4.1 The ledger

`stingy4rust::ledger::ledger::Ledger<const SCOPES: usize, const BLOCKS: usize, const NAMES: usize>`
holds every count of a run in fixed-size tables. Its counting path -- every
method an allocator call reaches -- **never allocates** and never panics
(ADR-CountingNeverAllocates): a table that fills sets an error the run reports,
and the counting carries on without it.

```rust
impl<const SCOPES: usize, const BLOCKS: usize, const NAMES: usize> Ledger<SCOPES, BLOCKS, NAMES> {
    pub const fn new() -> Self;

    // The allocator's calls. `thread` is the calling thread's mark (4.4).
    pub fn allocated(&self, address: usize, size: usize, thread: usize);
    pub fn freed(&self, address: usize, size: usize, thread: usize);
    pub fn reallocated(&self, old_address: usize, old_size: usize,
                       new_address: usize, new_size: usize, thread: usize);

    // A run: counting starts at `begin` and stops at `end`, which hands back
    // what was counted. `end` may allocate: it runs once counting has stopped.
    pub fn begin(&self, thread: usize);
    pub fn end(&self) -> Tally;

    // The bookkeeping's own cost, reported, never budgeted.
    pub fn bookkeeping(&self) -> Bookkeeping;
}
```

`Ledger` also implements the `Recorder` trait (section 5), through which a
`Run`, its guards and its meters reach it. Outside a run -- before `begin`,
after `end` -- every call returns at once: a process allocates freely while it
starts and while it writes its report.

- **BLOCKS** is a power of two (a `const` assertion), the capacity of the
  table of live blocks: open addressing, linear probing, keyed by address, with
  backward-shift deletion so no tombstone ever fills it. Each entry holds the
  address, the size and the index of the scope charged.
- **SCOPES** bounds the distinct scope names of a run, `(outside scopes)`
  included. **NAMES** bounds the distinct wire kinds, stores and operations
  together.
- **Nesting** is bounded at 64 open scopes, a constant of the core
  (`MAX_DEPTH`); deeper is an error the run reports.

### 4.2 Scopes

A scope is named by `ScopeName { name: &'static str, number: Option<u32> }` --
`node`, or `node 3` -- so naming one never allocates. A run opens one with
`run.scope("leader step")` or `run.scope_numbered("node", 3)`, which returns a
`ScopeGuard`; until the guard drops, every allocation, reallocation and peak is
charged to that scope -- the innermost one, if they nest. A harness opens one
around each component's step, so the figures are the code's, never the
harness's, which allocates around the steps to drive them.

- **Freed elsewhere is credited back exactly** (ADR-FreedElsewhereIsCreditedExactly):
  the table of live blocks keeps the scope each block was charged to, and a
  free -- whoever does it -- comes off that scope. A reallocation frees the old
  block from its scope and charges the new one to the scope open now.
- **A free the table never saw** -- a block allocated before `begin` -- is
  counted in the bookkeeping as an untracked free and changes no figure.
- **A guard dropped out of order** -- an outer guard before an inner one -- is
  an error the run reports, not a figure quietly misplaced.
- **A name may not contain `.`**: it would break the figure's path (7.1). Such
  a name is an error the run reports.

### 4.3 The lock

The ledger's tables sit behind a `spin::Mutex`, constructible in a `const fn`
and taken without allocating. A run measures one thread (4.4), so the lock is
never contended while counting; it is what makes the ledger `Sync`, which a
`static` allocator must be, without one line of `unsafe` in the core.

### 4.4 One thread

A run is measured on one thread (ADR-OneThreadWhileMeasuring). `begin` records
the mark of the thread that calls it; while a scope is open, an allocator call
from any other thread sets the run's `ForeignThread` error and is not counted.
On a host the mark is the address of a `const`-initialised thread-local --
taking it allocates nothing; on a board there is one thread and the mark is
constant.

### 4.5 What a run's tally holds

```rust
pub struct Tally {
    pub status: RunStatus,
    pub heap: HeapFigures,           // the run's totals
    pub peak: u64,                   // the run's peak
    pub scopes: Vec<ScopeFigures>,   // in the order first entered, (outside scopes) first
    pub wire: Vec<WireFigures>,      // in the order first recorded
    pub storage: Vec<StoreFigures>,
    pub operations: Vec<OperationFigures>,
    pub bookkeeping: Bookkeeping,
}

pub enum RunStatus {
    Measured,
    ScopesExhausted,      // more distinct scope names than SCOPES
    BlocksExhausted,      // more live blocks than BLOCKS
    NamesExhausted,       // more kinds, stores and operations than NAMES
    DepthExhausted,       // more than MAX_DEPTH scopes open at once
    ForeignThread,        // another thread allocated while a scope was open
    ScopeLeftOutOfOrder,  // a guard dropped before one it encloses
    NameWithDot,          // a scope, kind, store or operation named with a '.'
}

pub struct HeapFigures { pub allocations: u64, pub reallocations: u64,
                         pub bytes_allocated: u64, pub bytes_freed: u64,
                         pub live_at_end: u64 }
pub struct ScopeFigures { pub name: ScopeName, pub heap: HeapFigures,
                          pub peak: u64, pub largest_entry_rise: u64, pub entries: u64 }
pub struct WireFigures { pub kind: &'static str, pub messages: u64, pub bytes: u64 }
pub struct StoreFigures { pub store: ScopeName, pub bytes_written: u64, pub writes: u64,
                          pub transactions: u64, pub bytes_at_rest: u64,
                          pub bytes_on_disk: Option<u64> }
pub struct OperationFigures { pub name: &'static str, pub count: u64,
                              pub messages: u64, pub bytes: u64 }
pub struct Bookkeeping { pub block_capacity: u64, pub tracked_blocks_peak: u64,
                         pub scope_capacity: u64, pub name_capacity: u64,
                         pub untracked_frees: u64, pub ledger_bytes: u64 }
```

Each is a record of its own file, as the house rules ask (section 12).

## 5. Runs, profiles and meters

### 5.1 The recorder

```rust
pub trait Recorder: Sync {
    fn enter(&self, scope: ScopeName) -> usize;     // the frame's depth
    fn leave(&self, depth: usize);
    fn begin_operation(&self, name: &'static str);
    fn end_operation(&self, name: &'static str);
    fn sent(&self, kind: &'static str, bytes: u64);
    fn written(&self, store: ScopeName, bytes: u64);
    fn transaction(&self, store: ScopeName);
    fn at_rest(&self, store: ScopeName, bytes: u64);
    fn on_disk(&self, store: ScopeName, bytes: u64);
}
```

`Ledger` implements it; tests implement it with a fake that keeps the calls.
Every handle below holds a `&'static dyn Recorder`, so it is `Copy` and
`'static` and can sit inside a project's boxed adapter.

### 5.2 A run and its profile

```rust
pub struct Profile {                 // a record: one line of the registry
    pub name: &'static str,
    pub seed: u64,
    pub body: fn(&Run),
}

impl Run {
    pub fn new(name: &'static str, seed: u64, recorder: &'static dyn Recorder) -> Self;
    pub fn name(&self) -> &'static str;
    pub fn seed(&self) -> u64;
    pub fn scope(&self, name: &'static str) -> ScopeGuard;
    pub fn scope_numbered(&self, name: &'static str, number: u32) -> ScopeGuard;
    pub fn operation(&self, name: &'static str) -> OperationGuard;
    pub fn wire(&self) -> WireMeter;
    pub fn storage(&self, store: &'static str) -> StorageMeter;
    pub fn storage_numbered(&self, store: &'static str, number: u32) -> StorageMeter;
}
```

Profiles are plain functions in a registry -- a `&'static [Profile]` -- with no
macro (ADR-ProfilesArePlainFunctions). A profile's body is a deterministic run
of the project's own system under a fixed load, from `run.seed()`, opening
scopes and operations as it goes:

```rust
fn steady_load(run: &Run) {
    let mut cluster = Cluster::new(5, run.seed(), run.wire());
    cluster.advance_settled();
    for _ in 0..1_000 {
        let _command = run.operation("command");
        cluster.submit_and_advance(10);
    }
}
```

### 5.3 Operations

An `OperationGuard` counts one operation of its name when it drops; the
messages and bytes the wire meter records while it is open are charged to it as
well, so a report gives messages and bytes per operation. Operations do not
nest: opening one while another is open charges the wire to the newer one until
it closes.

### 5.4 Meters, and the project's decorators

stingy4rust cannot know a project's storage or transport traits. It gives the
meters, and the project writes a thin decorator over its own port that calls
them (the pattern: `docs/DECORATORS.md`, written in slice 4):

```rust
impl<T: Transport> Transport for Metered<T> {
    fn send(&mut self, to: PeerId, message: Message) {
        self.meter.sent(message.kind_name(), message.encoded_len() as u64);
        self.inner.send(to, message)
    }
}
```

```rust
impl WireMeter {
    pub fn sent(&self, kind: &'static str, bytes: u64);
}
impl StorageMeter {
    pub fn written(&self, bytes: u64);    // one write of `bytes`
    pub fn transaction(&self);            // one sync or commit
    pub fn at_rest(&self, bytes: u64);    // what the store holds, encoded, at the end
    pub fn on_disk(&self, bytes: u64);    // its files' size, where it has a disk
}
```

At rest and on disk differ by the store's own amplification -- pages, journals
-- which is the store's, not the code's; both are kept. Because every adapter
behind a port is held to one contract in the projects this is built for, one
decorator a port meters every adapter.

## 6. How the subcommand runs a project's profiles

**The profile target is a test without a harness** (ADR-TheProfileTargetIsATestWithoutHarness).
A project declares, in the package that holds its profiles:

```toml
[[test]]
name = "stingy"
path = "tests/stingy.rs"
harness = false

[dev-dependencies]
stingy4rust-core = "0.1"
stingy4rust-host = "0.1"
```

and writes its `main`:

```rust
use stingy4rust::run::profile::Profile;
use stingy4rust::run::run::Run;
use stingy4rust_host::file_report_sink::FileReportSink;
use stingy4rust_host::host_stingy::HostStingy;
use stingy4rust_host::process_environment::ProcessEnvironment;
use stingy4rust_host::profile_main::ProfileMain;
use stingy4rust_host::stingy::Stingy;
use std::alloc::System;
use std::process::ExitCode;

#[global_allocator]
static STINGY: HostStingy = Stingy::new(System);

static PROFILES: [Profile; 2] = [
    Profile { name: "five-nodes-idle", seed: 2026, body: idle },
    Profile { name: "five-nodes-100-commands-a-second", seed: 2026, body: steady_load },
];

fn main() -> ExitCode {
    ProfileMain::new(&STINGY, &PROFILES, &ProcessEnvironment, &FileReportSink).run()
}
```

- **Why a test target**: it is a dev-only target, so it may use the project's
  test fakes and dev-dependencies, is never shipped, and owns its
  `#[global_allocator]` without imposing it on the library. `harness = false`
  makes its `main` the project's own: no test harness threads, no output
  capture, one thread.
- **`HostStingy`** is the type `Stingy<System, 128, 262_144, 256>` -- 128 scope names,
  262,144 live blocks, 256 kinds, stores and operations; a project that needs
  more names its own `Stingy<System, S, B, N>`. Its table costs some 6 MiB of
  static memory on a 64-bit host, reported in the bookkeeping.
- **`ProfileMain::run`** reads, through its `Environment` port,
  `STINGY4RUST_REPORT` (where to write the report; required) and
  `STINGY4RUST_PROFILE` (one profile's name; optional), runs each profile in
  order on the main thread -- `begin`, the body, `end` -- hands the report to its
  `ReportSink` port, and exits 0 if every run is `Measured`, 2 otherwise, the
  report written either way. `ProcessEnvironment` and `FileReportSink` are the
  host's adapters; tests give fakes. `ProfileMain` reaches the allocator through
  the core's `Measuring` trait -- `Recorder` plus `begin` and `end` -- which
  `Stingy` implements, so the board crate's allocator plugs in the same way.
- **The subcommand** finds the package with a test target named `stingy` from
  `cargo metadata --format-version 1 --no-deps` (`--package` chooses among
  several; none is an error), then runs
  `cargo test --release --package <package> --test stingy`
  with `STINGY4RUST_REPORT=<workspace>/target/stingy4rust/<package>.json`, and
  `STINGY4RUST_PROFILE` when `--profile` is given. Release, because what ships
  is what is measured. The child's output passes through to the subcommand's
  standard error.
- **The target triple** a budget is kept under is the host's, from
  `rustc -vV`, or the one `--target` names, passed through to cargo.

## 7. The report, budgets and the gate

### 7.1 Figures and their paths

Every figure has a path, its parts joined by `.` -- which is why a name may not
hold one:

| path | figure |
|---|---|
| `heap.allocations`, `heap.reallocations`, `heap.bytes_allocated`, `heap.bytes_freed`, `heap.live_at_end` | the run's totals |
| `peaks.run` | the run's peak |
| `scopes.<scope>.<figure>` | `allocations`, `reallocations`, `bytes_allocated`, `bytes_freed`, `live_at_end`, `peak`, `largest_entry_rise`, `entries` |
| `storage.<store>.<figure>` | `bytes_written`, `writes`, `transactions`, `bytes_at_rest`, `bytes_on_disk` (only where reported) |
| `wire.<kind>.<figure>` | `messages`, `bytes` |
| `operations.<name>.<figure>` | `count`, `messages`, `bytes` |

A numbered name is written with its number: `scopes.node 3.peak`.

### 7.2 The report

The profile target writes it, with the core's own JSON writer -- no
serialisation crate in the core, keys in a fixed order, so the same run writes
the same bytes:

```json
{
  "stingy4rust": "report",
  "version": 1,
  "profiles": [
    {
      "name": "five-nodes-idle",
      "seed": 2026,
      "status": "measured",
      "figures": {
        "heap.allocations": 62730,
        "peaks.run": 47900,
        "scopes.node 1.peak": 9100
      },
      "bookkeeping": {
        "block_capacity": 262144, "tracked_blocks_peak": 812,
        "scope_capacity": 128, "name_capacity": 256,
        "untracked_frees": 0, "ledger_bytes": 6291456
      }
    }
  ]
}
```

`status` is the run's `RunStatus` in kebab case (`measured`,
`blocks-exhausted`, `foreign-thread`, ...). `figures` holds every figure of the
run, in the order of 7.1, scopes and the rest in the order first met.

### 7.3 The budget

One file at the root of the project's workspace, `stingy4rust-budget.json`
(ADR-OneBudgetFileAtTheRoot), keyed by target triple, then profile
(ADR-FiguresAreFlatPaths):

```json
{
  "stingy4rust": "budget",
  "version": 1,
  "targets": {
    "x86_64-unknown-linux-gnu": {
      "five-nodes-idle": {
        "figures": {
          "heap.allocations": 62730,
          "peaks.run": 47900
        },
        "tolerances": {
          "storage.node 1 log.bytes_on_disk": { "absolute": 4096 }
        }
      }
    }
  }
}
```

**Tolerance is zero by default** (ADR-ToleranceIsZeroAndPerFigure). A figure
may carry `{ "absolute": n }`, `{ "relative": r }` or both; the figure may then
rise to `budget + max(n, ceil(budget * r))`. There is no blanket tolerance:
a figure that is not deterministic says so on its own line, with its reason in
the commit that adds it.

### 7.4 Comparing

For the target being run, every profile, every figure:

| case | verdict | fails |
|---|---|---|
| measured equals the budget, or rises within its tolerance | `within` | no |
| measured below the budget | `gain` -- reported, with a hint to record it | no |
| measured above the budget plus its tolerance | `over` | yes |
| a figure measured with no budget | `unbudgeted` | yes |
| a figure budgeted and not measured | `missing` | yes |
| a profile measured with no budget, or budgeted and not run | `unbudgeted` / `missing` | yes |
| the target has no budget at all | `unbudgeted` | yes |

With `--profile`, only that profile is compared. Nothing goes unmeasured by
omission, and a figure that disappears -- a scope no longer entered -- fails
until it is recorded away, deliberately.

### 7.5 Commands, outputs, exit codes

```
cargo stingy4rust report [--package P] [--profile NAME] [--target T] [--format table|json]
cargo stingy4rust check  [--package P] [--profile NAME] [--target T] [--format table|json] [--budget PATH]
cargo stingy4rust record [--package P] [--profile NAME] [--target T] [--budget PATH]
```

- `report` runs the profiles and prints their figures; no budget is read.
- `check` runs them and compares; it never writes.
- `record` runs them and writes their figures as the budget of this target --
  every profile, or the one named -- keeping the tolerances of figures still
  measured, dropping those of figures gone, and printing what moved. Recording
  is a deliberate act, as re-recording a baseline is: the gate never records on
  its own.
- **Table** (the default): per profile, one row a figure -- path, budget,
  measured, difference, verdict -- then one line: within, gains, over.
- **JSON**: one document a run (as the family's tools do):
  `{"stingy4rust":"check","version":1,"target":"...","verdict":"within|over","profiles":[{"name":"...","figures":[{"path":"...","budget":n,"measured":n,"allowed":n,"verdict":"within"}]}]}`.
- **Exit codes** (ADR-ExitCodesSeparateVerdictFromFailure): **0** within budget
  (gains included); **1** over budget, unbudgeted or missing; **2** unable to
  measure -- no profile target, a build that failed, a run whose status is not
  `measured`, a report or a budget that cannot be read. A 2 is never a verdict.

## 8. Determinism

The figures are only a gate if they repeat. stingy4rust requires, and checks
what it can:

- **One thread** while scopes are open -- checked (4.4).
- **A fixed seed** for everything random, which the profile takes from
  `run.seed()`.
- **No hash map with a random state** in the code measured: a `HashMap`'s
  allocation pattern changes with its seed. The tool cannot see it; the
  project's own rules can.
- **A pinned toolchain**: a compiler release can change how a `Vec` grows. A
  toolchain bump re-records the budgets in its own commit.
- **Release builds**: the subcommand builds the profile target with `--release`.

## 9. Microcontrollers (slice 6)

- `stingy4rust-board` wraps the board's own `GlobalAlloc` and keeps the
  scope a block is charged to in a **header** it reserves in front of each
  block, so a board needs no table sized for its worst case; the header's cost
  is reported in the bookkeeping, apart from the figures. It also reports what
  the board's allocator really holds, where that allocator can say.
- The report goes out over the channel the board already has -- semihosting
  first, as QEMU gives it, then defmt -- as the same JSON a host writes, which
  the subcommand reads back.
- Budgets are per target triple: the same profile on a 32-bit board and on a
  64-bit host gives different figures, and a run on QEMU is a target of its
  own.

## 10. Errors the tool reports, not figures

Everything that would make a figure wrong is an error that stops the gate with
exit 2, never a figure quietly off: a table full, nesting too deep, another
thread allocating, a guard dropped out of order, a name with a `.`, a report
that does not parse, a budget that does not parse or names an unknown version.

## 11. How the tool holds itself

As the rest of the family: test-first; every house gate (stern, shear, dry,
crap, twin, iceberg, deny) on its own code; the core technology-free and
`no_std`, checked by the xtask's own gate; and a **fixture** crate whose
profiles allocate, send and store exactly known amounts -- a profile that
allocates three blocks of 64 bytes in a scope must report exactly that -- whose
figures are held to the fixture's own budget, so the tool gates itself before
it gates anyone.

## 12. The house's shape, as it bears on the design

- One object a file; a record (a struct of public fields and derives, no
  behaviour) is a file of its own and gets no test of its own.
- No `&mut` parameters but `&mut self`: hence `fn(&Run)` for a profile's body
  and handles that record through a shared `&'static dyn Recorder`.
- Dependencies injected through the one constructor: `Run::new` takes its
  recorder; `ProfileMain::new` its allocator and its profiles; the subcommand's
  parts take their ports (a command runner, a file store) as traits, with fakes
  in the tests.

## 13. Decisions of 2026-11-21

The proposal of 2026-11-20 left five questions; the owner answered the first
four, the fifth was settled with it.

1. **Profiles are plain functions in a registry**, not an attribute macro:
   one line a profile, nothing generated, no third crate to build, test and
   release. (ADR-ProfilesArePlainFunctions)
2. **A block freed in another scope is credited back exactly** to the scope
   that allocated it, by a per-live-block record: a table on a host, a header
   on a board; the bookkeeping's own cost is measured apart, never budgeted.
   In a protocol whose messages travel between components, blocks cross scopes
   all the time, and an approximation would put figures in the wrong place or
   below zero. (ADR-FreedElsewhereIsCreditedExactly)
3. **Tolerance is zero by default**, overridden per figure in the budget file,
   never as a blanket: the runs are deterministic, so any growth is real, and a
   blanket margin lets a regression in, a percent at a time.
   (ADR-ToleranceIsZeroAndPerFigure)
4. **Time is out entirely**: only what the code decides, identical on every
   machine. (ADR-TimeIsNotMeasured)
5. **One budget file at the root**, `stingy4rust-budget.json`, as the family's
   baselines are one file at the root. (ADR-OneBudgetFileAtTheRoot)

## 14. Confirmed 2026-11-22

The design made four calls of its own beyond those questions; the owner
confirmed all four.

1. **A third published crate, `stingy4rust-host`**, holds the host's
   `GlobalAlloc` and with it the only `unsafe` code; `stingy4rust-board`
   replaces it in slice 6. Both names are reserved by the owner before the
   first release that needs them. (ADR-TechnologyFreeSafeCore)
2. **The profile target is a test named `stingy`** with `harness = false`,
   driven by `STINGY4RUST_REPORT` and `STINGY4RUST_PROFILE`.
   (ADR-TheProfileTargetIsATestWithoutHarness)
3. **The first release is 0.1.0, after slice 4**: the host tool complete.
4. **The fixture's figures depend on the compiler**: they assume a `Vec<u8>`
   grows 8, 16, 32, 64, 128. A figure that differs is explained, never
   matched; if the toolchain genuinely grows otherwise, the owner decides.
