# ADR-BytesRequestedNotBytesUsed

## Status

- **Status:** Accepted
- **Date:** 2026-11-21

## Context

An allocation of 24 bytes may cost an allocator 32 or 48 -- a header, an
alignment, a size class. Which is the figure?

## Decision

The heap figures count the bytes requested, `Layout::size()`. What an allocator
adds is reported apart, by the backend that knows its allocator, and is never
budgeted.

## Forcing constraints / Evidence

The overhead depends on the allocator -- the system's on a host, the board's on
a board -- and on its version; the bytes requested depend only on the code. A
budget must move only when the code does.

## Rejected alternatives

- **Bytes as the allocator reports them**: budgets that change with the host's
  C library.

## Consequences

A board's report shows both the bytes requested and, where its allocator can
say, what it really holds.
