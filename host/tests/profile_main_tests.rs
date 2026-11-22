// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fake_environment::FakeEnvironment;
use crate::support::fake_measuring::FakeMeasuring;
use crate::support::fake_report_sink::FakeReportSink;
use std::process::ExitCode;
use stingy4rust::ledger::run_status::RunStatus;
use stingy4rust::run::profile::Profile;
use stingy4rust::run::run::Run;
use stingy4rust_host::profile_main::ProfileMain;

static PROFILES: [Profile; 2] = [
    Profile {
        name: "first",
        seed: 1,
        body: entered,
    },
    Profile {
        name: "second",
        seed: 2,
        body: entered,
    },
];
const REPORT: &str = "/reports/measured.json";

fn entered(run: &Run) {
    let _scope = run.scope(run.name());
}

fn leaked<T>(value: T) -> &'static T {
    Box::leak(Box::new(value))
}

#[test]
fn run_of_a_profile_whose_status_is_not_measured_exits_2_and_still_writes() {
    // Arrange
    let measuring = leaked(FakeMeasuring::new(&[
        RunStatus::Measured,
        RunStatus::BlocksExhausted,
    ]));
    let environment = leaked(FakeEnvironment::new().with("STINGY4RUST_REPORT", REPORT));
    let sink = leaked(FakeReportSink::new());

    // Act
    let code = ProfileMain::new(measuring, &PROFILES, environment, sink).run();

    // Assert
    assert_eq!(code, ExitCode::from(2));
    let written = sink.written();
    assert_eq!(written.len(), 1);
    assert!(written[0].1.contains("\"status\": \"blocks-exhausted\""));
}

#[test]
fn run_of_two_profiles_writes_both_to_the_path_named() {
    // Arrange
    let measuring = leaked(FakeMeasuring::new(&[]));
    let environment = leaked(FakeEnvironment::new().with("STINGY4RUST_REPORT", REPORT));
    let sink = leaked(FakeReportSink::new());

    // Act
    let code = ProfileMain::new(measuring, &PROFILES, environment, sink).run();

    // Assert
    assert_eq!(code, ExitCode::SUCCESS);
    assert_eq!(
        measuring.calls(),
        vec![
            "begin",
            "enter first",
            "end",
            "begin",
            "enter second",
            "end"
        ]
    );
    let written = sink.written();
    assert_eq!(written.len(), 1);
    assert_eq!(written[0].0, REPORT);
    assert!(written[0].1.contains("\"name\": \"first\""));
    assert!(written[0].1.contains("\"name\": \"second\""));
}

#[test]
fn run_whose_report_cannot_be_written_exits_2() {
    // Arrange
    let measuring = leaked(FakeMeasuring::new(&[]));
    let environment = leaked(FakeEnvironment::new().with("STINGY4RUST_REPORT", REPORT));
    let sink = leaked(FakeReportSink::refusing());

    // Act
    let code = ProfileMain::new(measuring, &PROFILES, environment, sink).run();

    // Assert
    assert_eq!(code, ExitCode::from(2));
    assert_eq!(measuring.calls().len(), 6);
}

#[test]
fn run_with_a_profile_named_runs_only_it() {
    // Arrange
    let measuring = leaked(FakeMeasuring::new(&[]));
    let environment = leaked(
        FakeEnvironment::new()
            .with("STINGY4RUST_REPORT", REPORT)
            .with("STINGY4RUST_PROFILE", "second"),
    );
    let sink = leaked(FakeReportSink::new());

    // Act
    let code = ProfileMain::new(measuring, &PROFILES, environment, sink).run();

    // Assert
    assert_eq!(code, ExitCode::SUCCESS);
    assert_eq!(measuring.calls(), vec!["begin", "enter second", "end"]);
    assert!(!sink.written()[0].1.contains("\"name\": \"first\""));
}

#[test]
fn run_with_an_unknown_profile_named_exits_2_and_runs_nothing() {
    // Arrange
    let measuring = leaked(FakeMeasuring::new(&[]));
    let environment = leaked(
        FakeEnvironment::new()
            .with("STINGY4RUST_REPORT", REPORT)
            .with("STINGY4RUST_PROFILE", "third"),
    );
    let sink = leaked(FakeReportSink::new());

    // Act
    let code = ProfileMain::new(measuring, &PROFILES, environment, sink).run();

    // Assert
    assert_eq!(code, ExitCode::from(2));
    assert_eq!(measuring.calls(), Vec::<String>::new());
    assert_eq!(sink.written(), Vec::new());
}

#[test]
fn run_with_no_report_path_exits_2_and_runs_nothing() {
    // Arrange
    let measuring = leaked(FakeMeasuring::new(&[]));
    let environment = leaked(FakeEnvironment::new());
    let sink = leaked(FakeReportSink::new());

    // Act
    let code = ProfileMain::new(measuring, &PROFILES, environment, sink).run();

    // Assert
    assert_eq!(code, ExitCode::from(2));
    assert_eq!(measuring.calls(), Vec::<String>::new());
    assert_eq!(sink.written(), Vec::new());
}
