// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::budget::check_outcome::CheckOutcome;
use stingy4rust::budget::figure_verdict::FigureVerdict;
use stingy4rust::budget::measured_profile::MeasuredProfile;
use stingy4rust::budget::profile_check::ProfileCheck;
use stingy4rust::budget::verdict::Verdict;
use stingy4rust::figures::figure::Figure;
use stingy4rust_cli::formats::reported_profile::ReportedProfile;
use stingy4rust_cli::formats::table::Table;

const TARGET: &str = "x86_64-unknown-linux-gnu";

fn figure(path: &str, value: u64) -> Figure {
    Figure {
        path: String::from(path),
        value,
    }
}

fn outcome() -> CheckOutcome {
    CheckOutcome {
        target: String::from(TARGET),
        profiles: vec![ProfileCheck {
            name: String::from("idle"),
            figures: vec![
                verdict("heap.allocations", Some(3), Some(3), Verdict::Within),
                verdict("peaks.run", Some(100), Some(99), Verdict::Gain),
                verdict("scopes.node 1.peak", Some(10), Some(12), Verdict::Over),
                verdict("scopes.new.peak", None, Some(5), Verdict::Unbudgeted),
                verdict("scopes.gone.peak", Some(4), None, Verdict::Missing),
            ],
        }],
    }
}

fn verdict(
    path: &str,
    budget: Option<u64>,
    measured: Option<u64>,
    verdict: Verdict,
) -> FigureVerdict {
    FigureVerdict {
        path: String::from(path),
        budget,
        measured,
        allowed: budget,
        verdict,
    }
}

#[test]
fn check_of_one_figure_in_each_verdict_is_the_documented_table() {
    // Arrange
    let outcome = outcome();

    // Act
    let text = Table::check(&outcome);

    // Assert
    assert_eq!(
        text,
        "target x86_64-unknown-linux-gnu\n\
         \n\
         profile idle\n\
         \x20 path                budget  measured  difference  verdict\n\
         \x20 heap.allocations         3         3           0  within\n\
         \x20 peaks.run              100        99          -1  gain\n\
         \x20 scopes.node 1.peak      10        12          +2  over\n\
         \x20 scopes.new.peak          -         5           -  unbudgeted\n\
         \x20 scopes.gone.peak         4         -           -  missing\n\
         \n\
         1 within, 1 gain, 1 over, 1 unbudgeted, 1 missing\n\
         a gain is kept by recording it: cargo stingy4rust record\n"
    );
}

#[test]
fn check_with_no_gain_gives_no_hint_to_record() {
    // Arrange
    let mut outcome = outcome();
    outcome.profiles[0].figures.truncate(1);

    // Act
    let text = Table::check(&outcome);

    // Assert
    assert!(
        text.ends_with("\n1 within, 0 gains, 0 over, 0 unbudgeted, 0 missing\n"),
        "{text}"
    );
}

#[test]
fn report_lists_each_profile_s_status_and_figures() {
    // Arrange
    let profiles = vec![ReportedProfile {
        status: String::from("measured"),
        measured: MeasuredProfile {
            name: String::from("three-blocks"),
            figures: vec![figure("heap.allocations", 3), figure("peaks.run", 192)],
        },
    }];

    // Act
    let text = Table::report(&profiles);

    // Assert
    assert_eq!(
        text,
        "profile three-blocks: measured\n\
         \x20 heap.allocations    3\n\
         \x20 peaks.run         192\n"
    );
}
