# stingy4rust -- the design

**Status:** proposed, 2026-11-20. Nothing here is built; the owner decides
each open question at the end before slice 1 begins.

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

Four families of figures, each counted under a **load profile** (section 5)
and attributed to **scopes** (section 3):

| family | figures |
|---|---|
| **heap** | allocations; reallocations; bytes requested; bytes freed; bytes live at the end |
| **peaks** | the high-water mark of bytes live -- of the run, of each scope, and the largest transient inside one scope entry (a step) |
| **storage** | bytes at rest at the end; bytes written; writes; transactions (syncs); bytes on disk, where a store has a disk |
| **wire** | messages sent and bytes sent, by kind of message; messages per operation the profile names |

What it does **not** measure: time. Durations depend on the machine and its
load; allocation counts, bytes and peaks do not, on a deterministic run with a
pinned toolchain (section 7). Timing belongs to a benchmark, beside it.

## 3. The heap: an allocator that counts, by scope

`stingy4rust-core` provides `Stingy`, a `GlobalAlloc` that wraps another --
the system allocator on a host, the board's on a microcontroller (section 8)
-- and counts every call before handing it on.

- **Bytes requested, not bytes used.** It counts `Layout::size()`. What an
  allocator adds on top -- headers, alignment, fragmentation -- differs from
  one allocator to the next, so it is measured separately, by the backend
  that knows its allocator, never mixed into the figures a budget holds.
- **Scopes.** The harness names what is running: `stingy::scope("node 3")`
  returns a guard, and every allocation, free and peak until it drops is
  attributed to that scope, nested scopes to the innermost. A harness opens one
  around each component's step -- a node, a kind of step, a request -- so the
  figures are the code's, never the harness's, which allocates around it to
  drive it. The scope stack is thread-local; a run measured with scopes is
  single-threaded (section 7).
- **Per-scope peaks.** Each scope keeps the high-water mark of the bytes live
  that it allocated and has not freed, across all its entries, and the largest
  rise inside one entry: a step that clones a snapshot six times shows there,
  even if every copy is freed before the step ends.
- **Freed elsewhere.** A block allocated in one scope and freed in another is
  credited back to the scope that allocated it: the allocator keeps, per live
  block, the scope it was charged to. On a host this costs a side table keyed
  by address; on a board a header the backend reserves (section 8).
- **Overhead.** The counting runs inside the allocator, so it must not
  allocate itself: fixed-size tables, sized by a const generic -- the number
  of scopes, the number of live blocks tracked -- and a run that overflows them
  fails loudly rather than miscount.

## 4. Storage and wire: decorators over the project's ports

stingy4rust cannot know a project's storage or transport traits. It provides
the **recorders** -- `StorageMeter`, `WireMeter` -- and the project writes a
thin decorator over its own port that calls them:

```rust
impl<S: StorageAdapter> StorageAdapter for Metered<S> {
    fn mutate(&mut self, mutation: Mutation) {
        self.meter.written(mutation.encoded_len());
        self.inner.mutate(mutation)
    }
    // ...
}
```

- **Storage** records bytes written, writes and transactions as they happen,
  and bytes at rest when the profile ends, as the project computes them from
  its store -- the encoded size of what it holds. A store with a disk adds its
  size on disk, read from its files, separately: the two differ by the store's
  own amplification (pages, logs), which is the store's, not the code's.
- **Wire** records each message's kind -- a name the project gives -- and its
  encoded bytes, as the transport hands it on; and the operations a profile
  names (`stingy::operation("command")`), so a report gives messages and bytes
  per operation.
- Because every adapter behind a port is held to the same contract in the
  projects this is built for, one decorator per port meters every adapter.

## 5. Load profiles

A profile is the project's own code: a deterministic run of its system under a
fixed load, from a fixed seed, that opens scopes and operations as it goes and
ends. stingy4rust runs it and keeps what it counted.

```rust
#[stingy::profile(name = "five-nodes-100-commands-a-second", seed = 2026)]
fn steady_load(run: &mut stingy::Run) {
    let mut cluster = Cluster::new(5, run.seed());
    cluster.advance_settled();
    for _ in 0..1_000 {
        let _command = run.operation("command");
        cluster.submit_and_advance(10);
    }
}
```

- A project gathers its profiles in one test target -- `tests/stingy.rs`, say
  -- which `cargo stingy4rust` builds in release and runs.
- Each profile states its configuration in its name and its code; the report
  carries the name and the seed, so a figure can always be reproduced.
- A project's first profiles are what its own assessment measured by hand: an
  idle system, steady loads at two rates, a long history, a component crashed,
  a large state transferred, a change of membership, a partition healed.

## 6. Budgets and the gate

`stingy4rust-budget.json`, checked in beside the code, holds for each target
and each profile the figures it was last recorded at:

```json
{
  "version": 1,
  "targets": {
    "x86_64-unknown-linux-gnu": {
      "five-nodes-100-commands-a-second": {
        "heap": { "allocations": 457111, "bytes_requested": 137600000, "live_at_end": 1440300 },
        "peaks": { "run": 1446300, "scopes": { "node 1": 312000 } },
        "storage": { "bytes_written": 98000, "transactions": 5000 },
        "wire": { "AppendEntries": { "messages": 6594, "bytes": 210000 } }
      }
    }
  },
  "tolerance": { "relative": 0.0, "absolute": 0 }
}
```

- `cargo stingy4rust check` runs every profile and compares: a figure above
  its budget beyond the tolerance **fails**; a figure below it is reported as
  a gain and **passes**; a profile or a figure with no budget fails, so nothing
  goes unmeasured by omission.
- `cargo stingy4rust record` writes the figures of this run as the budget.
  Recording is a deliberate act -- a change admitted, a gain kept -- as a
  baseline is: the gate never records on its own.
- **Tolerance zero by default.** On a deterministic run the figures repeat
  exactly; a tolerance is for what is not deterministic, stated per figure
  where it is needed, never a blanket.
- **Reports**: a table for a person -- each profile, each figure, budget,
  measured, the difference -- and `--format json` for a gate script, with exit
  codes: 0 within budget, 1 over budget, 2 unable to run.
- In the projects this is built for, it is a stage 2 gate beside the other
  tools of the family.

## 7. Determinism

The figures are only a gate if they repeat. stingy4rust requires, and checks
what it can:

- **One thread** while scopes are open; a second thread allocating inside a
  run is reported as an error, not counted.
- **A fixed seed** for everything random, which the profile takes from `Run`.
- **No hash map with a random state** in the code measured -- the allocation
  pattern of a `HashMap` changes with its seed. The tool cannot see it; the
  project's own rules can.
- **A pinned toolchain**: a compiler release can change how a `Vec` grows. A
  toolchain bump re-records the budgets in its own commit.
- **Release builds**: what ships is what is measured.

## 8. Microcontrollers

`stingy4rust-core` is `no_std`, its counting free of `std` and of allocation.

- **Backends**: on a host, `System`; on a board, a feature wrapping whatever
  `GlobalAlloc` the board uses, which also reports what that allocator really
  uses -- its own high-water mark and its overhead -- beside the bytes
  requested.
- **Reporting** from a board goes out over the channel the board already has
  (defmt, semihosting, a serial line) as the same JSON a host writes, which
  the subcommand reads back.
- **Budgets per target**: the same profile on a 32-bit board and a 64-bit
  host gives different figures; the budget file keys them by target triple,
  and a run on QEMU is a target of its own.

## 9. The crates

| crate | holds | `no_std` |
|---|---|---|
| `stingy4rust-core` | the counting allocator, scopes, peaks, `StorageMeter`, `WireMeter`, `Run` and operations, the report's data model and JSON | yes |
| `cargo-stingy4rust` | the subcommand: build and run the profile target, read the report, compare with the budget, print, record | no |
| a macro crate, if `#[stingy::profile]` needs one | the profile attribute | -- |

The family's split: the library and the subcommand released together under
one version.

## 10. How it is built and held

As the rest of the family: test-first, every rule of the house gates (stern,
dry, crap, twin, iceberg, deny) on its own code, a validation package with a
fixture crate whose allocation patterns are known exactly -- a profile that
allocates 3 blocks of 64 bytes in a scope must report exactly that -- and the
measured figures of the fixture itself held to a budget, so the tool gates
itself first.

## 11. Slices

1. **The counting allocator and scopes**: counts, bytes, live, peaks of the run
   and of each scope and scope entry; host backend; the fixture's exact
   figures.
2. **Profiles and the report**: `Run`, operations, the profile target, the
   JSON report, the table.
3. **Budgets and the gate**: the budget file, `check` and `record`, tolerance,
   exit codes; the tool's own budget.
4. **Storage and wire meters**, and the decorators' pattern documented.
5. **The first consumer's profiles**, its budgets recorded before its fixes.
6. **Microcontrollers**: the board backend, reporting over the board's channel,
   budgets per target, QEMU first.

## 12. Open questions for the owner

1. **Profiles as an attribute macro** (`#[stingy::profile]`, a third crate) or
   as plain functions registered in a list -- simpler, no macro crate.
2. **Tracking freed-elsewhere blocks**: exact (a side table per live block) or
   approximate (credited to the scope that frees it) -- exact costs memory per
   live block, which on a board is the thing measured.
3. **Tolerance**: zero by default as proposed, or a small relative margin to
   absorb toolchain noise without re-recording.
4. **Where the budgets live**: one file at the root, or one per profile beside
   its code.
5. **Time**: kept out entirely, as proposed, or a non-gating timing column for
   information.
