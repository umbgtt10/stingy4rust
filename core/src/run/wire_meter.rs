// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::run::recorder::Recorder;

// What a project's transport decorator calls for each message it sends: the
// kind of message and its encoded bytes.
#[derive(Clone, Copy)]
pub struct WireMeter {
    recorder: &'static dyn Recorder,
}

impl WireMeter {
    #[must_use]
    pub const fn new(recorder: &'static dyn Recorder) -> Self {
        Self { recorder }
    }

    pub fn sent(&self, kind: &'static str, bytes: u64) {
        self.recorder.sent(kind, bytes);
    }
}
