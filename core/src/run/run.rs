// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::figures::scope_name::ScopeName;
use crate::run::operation_guard::OperationGuard;
use crate::run::recorder::Recorder;
use crate::run::scope_guard::ScopeGuard;
use crate::run::storage_meter::StorageMeter;
use crate::run::wire_meter::WireMeter;

// One run of a load profile, handed to its body: its name and seed, and the
// handles through which the body opens scopes and operations and meters its
// wire and its stores -- each holding the run's recorder, so each is `'static`
// and can sit inside a project's own adapters.
pub struct Run {
    name: &'static str,
    seed: u64,
    recorder: &'static dyn Recorder,
}

impl Run {
    #[must_use]
    pub const fn new(name: &'static str, seed: u64, recorder: &'static dyn Recorder) -> Self {
        Self {
            name,
            seed,
            recorder,
        }
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    // Opens an operation of this name; it is counted, and closed, when the
    // guard drops.
    #[must_use]
    pub fn operation(&self, name: &'static str) -> OperationGuard {
        self.recorder.begin_operation(name);
        OperationGuard::new(self.recorder, name)
    }

    // Opens a scope of this name; what is allocated until the guard drops is
    // charged to it.
    #[must_use]
    pub fn scope(&self, name: &'static str) -> ScopeGuard {
        self.entered(ScopeName { name, number: None })
    }

    #[must_use]
    pub fn scope_numbered(&self, name: &'static str, number: u32) -> ScopeGuard {
        self.entered(ScopeName {
            name,
            number: Some(number),
        })
    }

    #[must_use]
    pub const fn seed(&self) -> u64 {
        self.seed
    }

    #[must_use]
    pub const fn storage(&self, store: &'static str) -> StorageMeter {
        StorageMeter::new(
            self.recorder,
            ScopeName {
                name: store,
                number: None,
            },
        )
    }

    #[must_use]
    pub const fn storage_numbered(&self, store: &'static str, number: u32) -> StorageMeter {
        StorageMeter::new(
            self.recorder,
            ScopeName {
                name: store,
                number: Some(number),
            },
        )
    }

    #[must_use]
    pub const fn wire(&self) -> WireMeter {
        WireMeter::new(self.recorder)
    }

    fn entered(&self, scope: ScopeName) -> ScopeGuard {
        ScopeGuard::new(self.recorder, self.recorder.enter(scope))
    }
}
