# ADR-OneBudgetFileAtTheRoot

## Status

- **Status:** Accepted
- **Date:** 2026-11-21

## Context

A project's budgets could live in one file, or one file a profile beside its
code.

## Decision

One file, `stingy4rust-budget.json`, at the root of the project's workspace,
keyed by target triple, then profile; `--budget PATH` names another.

## Forcing constraints / Evidence

Settled with the owner's answers of 2026-11-21: the family's baselines --
`dry4rust-baseline.json`, `stern4rust-baseline.json` -- are each one file at the
root, recorded by one command and reviewed as one diff. A budget recorded for
every profile at once is one change to read.

## Rejected alternatives

- **One file a profile, beside its code**: a re-recording touches as many
  files as profiles, and a profile removed leaves its file behind.

## Consequences

The file grows with targets and profiles; its keys are written sorted, so a
re-recording's diff shows only the figures that moved.
