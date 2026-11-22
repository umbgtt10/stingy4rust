// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::sync::Mutex;
use stingy4rust_host::report_sink::ReportSink;
use stingy4rust_host::sink_failure::SinkFailure;

// A sink that keeps every report it is handed, with its path, or refuses
// every one.
pub struct FakeReportSink {
    written: Mutex<Vec<(String, String)>>,
    refuses: bool,
}

impl FakeReportSink {
    pub const fn new() -> Self {
        Self {
            written: Mutex::new(Vec::new()),
            refuses: false,
        }
    }

    pub const fn refusing() -> Self {
        Self {
            written: Mutex::new(Vec::new()),
            refuses: true,
        }
    }

    pub fn written(&self) -> Vec<(String, String)> {
        self.written
            .lock()
            .expect("the reports are not poisoned")
            .clone()
    }
}

impl ReportSink for FakeReportSink {
    fn write(&self, path: &str, text: &str) -> Result<(), SinkFailure> {
        if self.refuses {
            return Err(SinkFailure {
                path: String::from(path),
                reason: String::from("refused by the fake"),
            });
        }
        self.written
            .lock()
            .expect("the reports are not poisoned")
            .push((String::from(path), String::from(text)));
        Ok(())
    }
}
