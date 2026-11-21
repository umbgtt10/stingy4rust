# ADR-TheProfileTargetIsATestWithoutHarness

## Status

- **Status:** Accepted
- **Date:** 2026-11-21

## Context

The subcommand must build and run a project's profiles in a process whose
global allocator counts, in release, on one thread, and get their figures back.

## Decision

The profiles live in a test target named `stingy`, declared with `harness =
false`, whose `main` sets `#[global_allocator]` to the host's counting allocator
and hands its registry to `ProfileMain`. The subcommand finds it with `cargo
metadata --no-deps`, runs `cargo test --release --package <p> --test stingy`
with `STINGY4RUST_REPORT` naming where the report goes (and
`STINGY4RUST_PROFILE` naming one profile, when asked), and reads the report the
target wrote.

## Forcing constraints / Evidence

A test target is dev-only: it can use the project's test fakes, is never
shipped, and its allocator does not reach the library. Without a harness, its
`main` is the project's: no harness threads, no captured output, one thread.
`--release` measures what ships. An environment variable passes a path without
a command line to parse in the measured process.

## Rejected alternatives

- **An example or a binary**: shipped with the package, or unable to use
  dev-dependencies.
- **A bench target**: built in release by default, but `cargo bench` brings a
  harness convention and a nightly-flavoured history.
- **Figures printed on standard output**: mixed with whatever the project
  prints; a file is unambiguous.

## Consequences

A project adds one `[[test]]` section and one file. The target's name is fixed
at `stingy`; a workspace with several measured packages is chosen from with
`--package`.
