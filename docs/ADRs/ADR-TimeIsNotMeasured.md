# ADR-TimeIsNotMeasured

## Status

- **Status:** Accepted
- **Date:** 2026-11-21

## Context

A tool that counts what a run costs could also time it, if only for
information.

## Decision

Time appears nowhere: not in the figures, not in the report, not as a
non-gating column. stingy4rust measures allocations, bytes, peaks, storage and
wire -- what the code decides.

## Forcing constraints / Evidence

The owner's answer of 2026-11-21. A duration differs between machines, between
runs on one machine, and with what else the machine is doing; the counts do
not. A column that differs between the owner's two laptops on the same commit
would be read as a change.

## Rejected alternatives

- **A non-gating timing column**: useful at a glance, and misleading in every
  comparison between two runs.

## Consequences

Timing belongs to a benchmark beside stingy4rust, run on an idle machine.
