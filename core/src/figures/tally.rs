// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::vec::Vec;

use crate::figures::heap_figures::HeapFigures;
use crate::figures::operation_figures::OperationFigures;
use crate::figures::scope_figures::ScopeFigures;
use crate::figures::store_figures::StoreFigures;
use crate::figures::wire_figures::WireFigures;
use crate::ledger::bookkeeping::Bookkeeping;
use crate::ledger::run_status::RunStatus;

// Everything one run counted, handed back when it ends: whether it could be
// measured, the run's heap and peak, each scope's figures -- `(outside scopes)`
// first, then in the order first entered -- the wire, the stores, the
// operations, and what the counting itself cost.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tally {
    pub status: RunStatus,
    pub heap: HeapFigures,
    pub peak: u64,
    pub scopes: Vec<ScopeFigures>,
    pub wire: Vec<WireFigures>,
    pub storage: Vec<StoreFigures>,
    pub operations: Vec<OperationFigures>,
    pub bookkeeping: Bookkeeping,
}
