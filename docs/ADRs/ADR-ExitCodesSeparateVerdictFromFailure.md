# ADR-ExitCodesSeparateVerdictFromFailure

## Status

- **Status:** Accepted
- **Date:** 2026-11-21

## Context

A gate script reads the subcommand's exit code. Over budget and unable to
measure are different news.

## Decision

0: every figure within its budget (gains included). 1: a figure over, a figure
or profile unbudgeted, or one missing. 2: unable to measure -- no profile
target, a build that failed, a run whose status is not `measured`, a report or a
budget that cannot be read. A 2 is never a verdict.

## Forcing constraints / Evidence

The family's tools keep the same split (dry4rust's exit 1 is its ceiling
exceeded; anything else is reported as the code it is), so a gate can say
"budget exceeded" only when it was.

## Rejected alternatives

- **One failure code**: a broken build would read as a regression.

## Consequences

The project's xtask gate decodes 1 as over budget and anything else as unable to
run.
