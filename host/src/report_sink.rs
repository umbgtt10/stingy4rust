// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::sink_failure::SinkFailure;

// Where a profile target writes its report: a file, or a test's fake.
pub trait ReportSink: Sync {
    fn write(&self, path: &str, text: &str) -> Result<(), SinkFailure>;
}
