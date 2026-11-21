# ADR-FreedElsewhereIsCreditedExactly

## Status

- **Status:** Accepted
- **Date:** 2026-11-21

## Context

Figures are attributed to scopes -- a component's step, a kind of step -- so a
report says which code allocated what. In a distributed protocol, blocks cross
scopes constantly: a message allocated in one node's step is freed in
another's, an entry built by a leader is dropped by a follower. Whose figures
does the free come off?

## Decision

The scope that allocated the block. The ledger keeps, for every live block, the
scope it was charged to; a free, whoever does it, is credited back to that
scope; a reallocation frees the old block from its scope and charges the new
one to the scope open at the reallocation. On a host the record is a fixed
table of live blocks; on a board a header the backend reserves in front of each
block. The record's own cost is reported in the bookkeeping, apart, and never
budgeted.

## Forcing constraints / Evidence

The owner's answer of 2026-11-21. Crediting the freeing scope instead would let
a scope's live bytes go below zero, and put a producer's cost on its consumer:
the figures would be wrong exactly where a protocol moves data between
components, which is where its cost is.

## Rejected alternatives

- **Credited to the scope that frees it**: no memory per block, figures wrong
  wherever blocks travel.
- **Tracking only counts, not ownership**: the run's totals stay right, every
  scope's live and peak figures do not.

## Consequences

The ledger needs a table sized for the most blocks live at once
(`BLOCKS`, a const generic); a run that outgrows it fails with
`BlocksExhausted` rather than miscount (ADR-CountingNeverAllocates). A free of
a block allocated before the run began is counted as an untracked free and
changes no figure.
