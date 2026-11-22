// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::figures::operation_figures::OperationFigures;
use crate::figures::scope_name::ScopeName;
use crate::figures::store_figures::StoreFigures;
use crate::figures::wire_figures::WireFigures;
use crate::ledger::name_event::NameEvent;
use crate::ledger::name_role::NameRole;

const EMPTY: NameSlot = NameSlot {
    role: NameRole::Kind,
    name: ScopeName {
        name: "",
        number: None,
    },
    counted: 0,
    bytes: 0,
    completed: 0,
    at_rest: 0,
    on_disk: None,
};

// One name of a run and what was counted against it, in counters its role
// reads: `counted` is a kind's or an operation's messages and a store's
// writes, `bytes` their bytes; `completed` is an operation's count and a
// store's transactions; `at_rest` and `on_disk` are a store's last values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NameSlot {
    pub role: NameRole,
    pub name: ScopeName,
    pub counted: u64,
    pub bytes: u64,
    pub completed: u64,
    pub at_rest: u64,
    pub on_disk: Option<u64>,
}

// A run's kinds of message, stores and operations, together, in the order
// first recorded: a fixed array, so recording one never allocates and a full
// table refuses the name. Every count saturates; a slot it does not hold is
// ignored.
pub struct NameTable<const NAMES: usize> {
    slots: [NameSlot; NAMES],
    len: usize,
}

impl<const NAMES: usize> NameTable<NAMES> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            slots: [EMPTY; NAMES],
            len: 0,
        }
    }

    pub const fn clear(&mut self) {
        self.len = 0;
    }

    // The slot of a name in a role: the one it was given when first met, or
    // the next free one; none once the table is full.
    pub fn intern(&mut self, role: NameRole, name: ScopeName) -> Option<usize> {
        if let Some(slot) = self.slots[..self.len]
            .iter()
            .position(|held| held.role == role && held.name == name)
        {
            return Some(slot);
        }
        let slot = self.len;
        *self.slots.get_mut(slot)? = NameSlot {
            role,
            name,
            ..EMPTY
        };
        self.len += 1;
        Some(slot)
    }

    #[must_use]
    pub fn operations(&self) -> Vec<OperationFigures> {
        self.of(NameRole::Operation)
            .map(|name| OperationFigures {
                name: name.name.name,
                count: name.completed,
                messages: name.counted,
                bytes: name.bytes,
            })
            .collect()
    }

    // What happened to the name in this slot, counted against it.
    pub fn record(&mut self, slot: usize, event: NameEvent) {
        let Some(name) = self.slots[..self.len].get_mut(slot) else {
            return;
        };
        match event {
            NameEvent::Counted(bytes) => {
                name.counted = name.counted.saturating_add(1);
                name.bytes = name.bytes.saturating_add(bytes);
            }
            NameEvent::Completed => name.completed = name.completed.saturating_add(1),
            NameEvent::AtRest(bytes) => name.at_rest = bytes,
            NameEvent::OnDisk(bytes) => name.on_disk = Some(bytes),
        }
    }

    #[must_use]
    pub fn storage(&self) -> Vec<StoreFigures> {
        self.of(NameRole::Store)
            .map(|name| StoreFigures {
                store: name.name,
                bytes_written: name.bytes,
                writes: name.counted,
                transactions: name.completed,
                bytes_at_rest: name.at_rest,
                bytes_on_disk: name.on_disk,
            })
            .collect()
    }

    #[must_use]
    pub fn wire(&self) -> Vec<WireFigures> {
        self.of(NameRole::Kind)
            .map(|name| WireFigures {
                kind: name.name.name,
                messages: name.counted,
                bytes: name.bytes,
            })
            .collect()
    }

    fn of(&self, role: NameRole) -> impl Iterator<Item = &NameSlot> {
        self.slots[..self.len]
            .iter()
            .filter(move |name| name.role == role)
    }
}

impl<const NAMES: usize> Default for NameTable<NAMES> {
    fn default() -> Self {
        Self::new()
    }
}
