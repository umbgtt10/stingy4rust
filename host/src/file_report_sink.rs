// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::fs::create_dir_all;
use std::fs::write;
use std::io;
use std::path::Path;

use crate::report_sink::ReportSink;
use crate::sink_failure::SinkFailure;

// Writes the report to a file, making the directories above it first:
// `target/stingy4rust/` need not exist before the first run.
pub struct FileReportSink;

impl FileReportSink {
    fn failure(path: &str, error: &io::Error) -> SinkFailure {
        SinkFailure {
            path: String::from(path),
            reason: error.to_string(),
        }
    }
}

impl ReportSink for FileReportSink {
    fn write(&self, path: &str, text: &str) -> Result<(), SinkFailure> {
        if let Some(parent) = Path::new(path).parent() {
            create_dir_all(parent).map_err(|error| Self::failure(path, &error))?;
        }
        write(path, text).map_err(|error| Self::failure(path, &error))
    }
}
