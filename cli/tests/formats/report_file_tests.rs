// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::budget::measured_profile::MeasuredProfile;
use stingy4rust::figures::figure::Figure;
use stingy4rust_cli::formats::report_file::ReportFile;
use stingy4rust_cli::formats::reported_profile::ReportedProfile;
use stingy4rust_cli::unable::Unable;

const REPORT: &str = r#"{
  "stingy4rust": "report",
  "version": 1,
  "profiles": [
    {
      "name": "three-blocks",
      "seed": 2026,
      "status": "measured",
      "figures": {
        "peaks.run": 192,
        "heap.allocations": 3
      },
      "bookkeeping": {
        "block_capacity": 8,
        "tracked_blocks_peak": 3,
        "scope_capacity": 4,
        "name_capacity": 4,
        "untracked_frees": 0,
        "ledger_bytes": 4096
      }
    },
    {
      "name": "overflowing",
      "seed": 2026,
      "status": "blocks-exhausted",
      "figures": {},
      "bookkeeping": {
        "block_capacity": 8,
        "tracked_blocks_peak": 8,
        "scope_capacity": 4,
        "name_capacity": 4,
        "untracked_frees": 0,
        "ledger_bytes": 4096
      }
    }
  ]
}
"#;

fn figure(path: &str, value: u64) -> Figure {
    Figure {
        path: String::from(path),
        value,
    }
}

#[test]
fn read_of_a_report_keeps_each_profile_s_status_and_figures_in_order() {
    // Arrange & Act
    let profiles = ReportFile::read(REPORT);

    // Assert
    assert_eq!(
        profiles,
        Ok(vec![
            ReportedProfile {
                status: String::from("measured"),
                measured: MeasuredProfile {
                    name: String::from("three-blocks"),
                    figures: vec![figure("peaks.run", 192), figure("heap.allocations", 3)],
                },
            },
            ReportedProfile {
                status: String::from("blocks-exhausted"),
                measured: MeasuredProfile {
                    name: String::from("overflowing"),
                    figures: Vec::new(),
                },
            },
        ])
    );
}

#[test]
fn read_of_a_report_of_an_unknown_version_fails() {
    // Arrange
    let text = REPORT.replace("\"version\": 1", "\"version\": 2");

    // Act
    let profiles = ReportFile::read(&text);

    // Assert
    assert_eq!(
        profiles,
        Err(Unable::UnreadableReport(String::from(
            "it is version 2; this stingy4rust reads version 1"
        )))
    );
}

#[test]
fn read_of_text_that_is_not_a_report_fails() {
    // Arrange
    let text = "{\"stingy4rust\": \"budget\", \"version\": 1, \"profiles\": []}";

    // Act
    let profiles = ReportFile::read(text);

    // Assert
    assert_eq!(
        profiles,
        Err(Unable::UnreadableReport(String::from(
            "it is a budget, not a report"
        )))
    );
    assert!(matches!(
        ReportFile::read("not json"),
        Err(Unable::UnreadableReport(_))
    ));
}
