// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::figures::heap_figures::HeapFigures;
use stingy4rust::figures::scope_figures::ScopeFigures;
use stingy4rust::figures::scope_name::ScopeName;
use stingy4rust::figures::tally::Tally;
use stingy4rust::ledger::bookkeeping::Bookkeeping;
use stingy4rust::ledger::run_status::RunStatus;
use stingy4rust::report::profile_report::ProfileReport;
use stingy4rust::report::report::Report;
use stingy4rust::report::report_json::ReportJson;

fn report_of(name: &'static str, tally: Tally) -> Report {
    Report {
        profiles: vec![ProfileReport {
            name,
            seed: 2026,
            tally,
        }],
    }
}

const fn scope(
    name: &'static str,
    number: Option<u32>,
    heap: HeapFigures,
    peak: u64,
) -> ScopeFigures {
    ScopeFigures {
        name: ScopeName { name, number },
        heap,
        peak,
        largest_entry_rise: peak,
        entries: 1,
    }
}

const fn tally(status: RunStatus, scopes: Vec<ScopeFigures>) -> Tally {
    Tally {
        status,
        heap: HeapFigures {
            allocations: 3,
            reallocations: 0,
            bytes_allocated: 192,
            bytes_freed: 192,
            live_at_end: 0,
        },
        peak: 192,
        scopes,
        wire: Vec::new(),
        storage: Vec::new(),
        operations: Vec::new(),
        bookkeeping: Bookkeeping {
            block_capacity: 8,
            tracked_blocks_peak: 3,
            scope_capacity: 4,
            name_capacity: 4,
            untracked_frees: 0,
            ledger_bytes: 4096,
        },
    }
}

#[test]
fn write_of_a_failed_run_writes_its_status_in_kebab_case() {
    // Arrange
    let statuses = [
        (RunStatus::Measured, "measured"),
        (RunStatus::ScopesExhausted, "scopes-exhausted"),
        (RunStatus::BlocksExhausted, "blocks-exhausted"),
        (RunStatus::NamesExhausted, "names-exhausted"),
        (RunStatus::DepthExhausted, "depth-exhausted"),
        (RunStatus::ForeignThread, "foreign-thread"),
        (RunStatus::ScopeLeftOutOfOrder, "scope-left-out-of-order"),
        (RunStatus::NameWithDot, "name-with-dot"),
    ];

    // Act
    let written: Vec<String> = statuses
        .iter()
        .map(|(status, _)| ReportJson::write(&report_of("profile", tally(*status, Vec::new()))))
        .collect();

    // Assert
    for ((_, name), json) in statuses.iter().zip(&written) {
        assert!(
            json.contains(&format!("\"status\": \"{name}\",")),
            "{name} missing from {json}"
        );
    }
}

#[test]
fn write_of_a_name_with_a_quote_escapes_it() {
    // Arrange
    let report = report_of(
        "say \"hi\"",
        tally(
            RunStatus::Measured,
            vec![scope("a\"b", None, HeapFigures::default(), 0)],
        ),
    );

    // Act
    let json = ReportJson::write(&report);

    // Assert
    assert!(json.contains("\"name\": \"say \\\"hi\\\"\","));
    assert!(json.contains("\"scopes.a\\\"b.peak\": 0,"));
}

#[test]
fn write_of_a_report_of_one_profile_is_the_documented_json() {
    // Arrange
    let worker = HeapFigures {
        allocations: 3,
        reallocations: 0,
        bytes_allocated: 192,
        bytes_freed: 192,
        live_at_end: 0,
    };
    let report = report_of(
        "three-blocks",
        tally(
            RunStatus::Measured,
            vec![scope("worker", None, worker, 192)],
        ),
    );

    // Act
    let json = ReportJson::write(&report);

    // Assert
    assert_eq!(
        json,
        r#"{
  "stingy4rust": "report",
  "version": 1,
  "profiles": [
    {
      "name": "three-blocks",
      "seed": 2026,
      "status": "measured",
      "figures": {
        "heap.allocations": 3,
        "heap.reallocations": 0,
        "heap.bytes_allocated": 192,
        "heap.bytes_freed": 192,
        "heap.live_at_end": 0,
        "peaks.run": 192,
        "scopes.worker.allocations": 3,
        "scopes.worker.reallocations": 0,
        "scopes.worker.bytes_allocated": 192,
        "scopes.worker.bytes_freed": 192,
        "scopes.worker.live_at_end": 0,
        "scopes.worker.peak": 192,
        "scopes.worker.largest_entry_rise": 192,
        "scopes.worker.entries": 1
      },
      "bookkeeping": {
        "block_capacity": 8,
        "tracked_blocks_peak": 3,
        "scope_capacity": 4,
        "name_capacity": 4,
        "untracked_frees": 0,
        "ledger_bytes": 4096
      }
    }
  ]
}
"#
    );
}

#[test]
fn write_of_two_profiles_lists_them_in_the_order_run() {
    // Arrange
    let mut report = report_of("first", tally(RunStatus::Measured, Vec::new()));
    report.profiles.push(ProfileReport {
        name: "second",
        seed: 7,
        tally: tally(RunStatus::Measured, Vec::new()),
    });

    // Act
    let json = ReportJson::write(&report);

    // Assert
    let first = json.find("\"name\": \"first\"").expect("first is written");
    let second = json
        .find("\"name\": \"second\"")
        .expect("second is written");
    assert!(first < second);
    assert!(json.contains("\"seed\": 7,"));
}
