# Architecture Decision Records

Each ADR documents one load-bearing decision behind `cargo-stingy4rust` --
succinct, self-contained, citable on its own. As in the sibling `*4rust`
tools, they are not priority-tiered: a few crates have a small enough decision
surface that a flat list is enough. The design they settle is in
[`../DESIGN.md`](../DESIGN.md); how it is built, in
[`../IMPLEMENTATION-PLAN.md`](../IMPLEMENTATION-PLAN.md).

## Index

| ADR | Decision |
|---|---|
| [ADR-ProfilesArePlainFunctions](ADR-ProfilesArePlainFunctions.md) | A profile is a plain `fn(&Run)` in a registry the project writes; no macro crate. (Owner, 2026-11-21.) |
| [ADR-FreedElsewhereIsCreditedExactly](ADR-FreedElsewhereIsCreditedExactly.md) | A block freed in another scope comes off the scope that allocated it, by a per-live-block record whose cost is reported apart. (Owner, 2026-11-21.) |
| [ADR-ToleranceIsZeroAndPerFigure](ADR-ToleranceIsZeroAndPerFigure.md) | Any growth fails unless that one figure carries a tolerance in the budget; gains pass and are reported. (Owner, 2026-11-21.) |
| [ADR-TimeIsNotMeasured](ADR-TimeIsNotMeasured.md) | No durations anywhere: only what the code decides. (Owner, 2026-11-21.) |
| [ADR-OneBudgetFileAtTheRoot](ADR-OneBudgetFileAtTheRoot.md) | One `stingy4rust-budget.json` at the workspace root, keyed by target, then profile. |
| [ADR-TechnologyFreeSafeCore](ADR-TechnologyFreeSafeCore.md) | The counting is a safe, `no_std`, technology-free core; the `unsafe` `GlobalAlloc` and the profile target's `main` are a host crate of their own. |
| [ADR-TheProfileTargetIsATestWithoutHarness](ADR-TheProfileTargetIsATestWithoutHarness.md) | Profiles run in a `harness = false` test target named `stingy`, in release, the report's path passed by environment variable. |
| [ADR-CountingNeverAllocates](ADR-CountingNeverAllocates.md) | Fixed tables sized by const generics; a table that fills is an error, never a quietly wrong figure. |
| [ADR-OneThreadWhileMeasuring](ADR-OneThreadWhileMeasuring.md) | Another thread allocating while a scope is open fails the run. |
| [ADR-BytesRequestedNotBytesUsed](ADR-BytesRequestedNotBytesUsed.md) | Heap figures count `Layout::size()`; an allocator's overhead is reported apart. |
| [ADR-FiguresAreFlatPaths](ADR-FiguresAreFlatPaths.md) | Every figure has a dotted path; budgets are flat maps keyed by target and profile; unbudgeted and missing both fail. |
| [ADR-ExitCodesSeparateVerdictFromFailure](ADR-ExitCodesSeparateVerdictFromFailure.md) | 0 within, 1 over, 2 unable to measure -- never a verdict. |
