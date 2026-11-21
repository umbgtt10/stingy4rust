# ADR-ProfilesArePlainFunctions

## Status

- **Status:** Accepted
- **Date:** 2026-11-21

## Context

A project measured by stingy4rust declares its load profiles: deterministic
runs of its own system under a fixed load, each with a name and a seed. The
proposal of 2026-11-20 sketched them as an attribute, `#[stingy::profile(name
= ..., seed = ...)]`, collected automatically, and asked the owner whether that
was worth a third crate.

## Decision

A profile is a plain function `fn(&Run)` listed in a registry the project
writes: a `&'static [Profile]`, where `Profile { name, seed, body }` is a
record. The profile target's `main` hands the registry to `ProfileMain`. There
is no macro crate.

## Forcing constraints / Evidence

The owner's answer of 2026-11-21. A registry is one line a profile, read as
written; an attribute macro is a proc-macro crate of its own -- built, tested,
released and versioned with the others -- and hides which profiles exist behind
a link-time collection that a reader cannot see in one place.

## Rejected alternatives

- **An attribute macro** (`#[stingy::profile]`): nicer to write one profile,
  a crate and a release more to keep, and collection by linker sections, which
  differs between hosts and boards.
- **Profiles discovered by name** (every `fn profile_*`): needs a macro or a
  build script all the same, and a misspelt name is silently not measured.

## Consequences

A profile forgotten in the registry is not run -- and the budget then holds it
`missing`, which fails the gate (ADR-FiguresAreFlatPaths), so the omission is
seen. Adding a profile is two edits in one file: the function and its line.
