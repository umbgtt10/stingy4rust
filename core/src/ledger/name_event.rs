// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

// What happens to a name in a run: one more message or write, and its bytes;
// one more operation or transaction completed; or a store's bytes at rest or
// on disk, the last value given kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameEvent {
    Counted(u64),
    Completed,
    AtRest(u64),
    OnDisk(u64),
}
