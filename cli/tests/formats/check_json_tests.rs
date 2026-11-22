// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::budget::check_outcome::CheckOutcome;
use stingy4rust::budget::figure_verdict::FigureVerdict;
use stingy4rust::budget::profile_check::ProfileCheck;
use stingy4rust::budget::verdict::Verdict;
use stingy4rust_cli::formats::check_json::CheckJson;

const TARGET: &str = "x86_64-unknown-linux-gnu";

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
fn write_of_a_check_with_nothing_failing_is_within() {
    // Arrange
    let mut outcome = outcome();
    outcome.profiles[0].figures.truncate(2);

    // Act
    let text = CheckJson::write(&outcome);

    // Assert
    assert!(text.contains(r#""verdict":"within","profiles""#), "{text}");
}

#[test]
fn write_of_a_check_with_one_figure_in_each_verdict_is_the_documented_json() {
    // Arrange
    let outcome = outcome();

    // Act
    let text = CheckJson::write(&outcome);

    // Assert
    assert_eq!(
        text,
        concat!(
            r#"{"stingy4rust":"check","version":1,"target":"x86_64-unknown-linux-gnu","verdict":"over","profiles":[{"name":"idle","figures":["#,
            r#"{"path":"heap.allocations","budget":3,"measured":3,"allowed":3,"verdict":"within"},"#,
            r#"{"path":"peaks.run","budget":100,"measured":99,"allowed":100,"verdict":"gain"},"#,
            r#"{"path":"scopes.node 1.peak","budget":10,"measured":12,"allowed":10,"verdict":"over"},"#,
            r#"{"path":"scopes.new.peak","budget":null,"measured":5,"allowed":null,"verdict":"unbudgeted"},"#,
            r#"{"path":"scopes.gone.peak","budget":4,"measured":null,"allowed":4,"verdict":"missing"}"#,
            "]}]}\n"
        )
    );
}
