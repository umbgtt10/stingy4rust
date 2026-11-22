// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::fake_command_runner::FakeCommandRunner;
use crate::support::fake_file_store::FakeFileStore;
use crate::support::sample::BUDGET_PATH;
use crate::support::sample::REPORT_PATH;
use crate::support::sample::RUSTC;
use crate::support::sample::budget;
use crate::support::sample::metadata;
use crate::support::sample::report;
use stingy4rust_cli::command_line::action::Action;
use stingy4rust_cli::command_line::command::Command;
use stingy4rust_cli::command_line::output_format::OutputFormat;
use stingy4rust_cli::entry_point::EntryPoint;
use stingy4rust_cli::formats::budget_file::BudgetFile;

const fn command(action: Action, format: OutputFormat) -> Command {
    Command {
        action,
        package: None,
        profile: None,
        target: None,
        format,
        budget: None,
    }
}

fn runner<'a>(
    store: &'a FakeFileStore,
    status: &str,
    test_code: i32,
    metadata_text: &str,
) -> FakeCommandRunner<'a> {
    FakeCommandRunner::new()
        .answering("cargo metadata", Some(0), metadata_text)
        .answering("rustc -vV", Some(0), RUSTC)
        .answering("cargo test", Some(test_code), "")
        .writing_report(store, REPORT_PATH, &report(status, 3))
}

#[test]
fn run_of_check_in_json_prints_one_document() {
    // Arrange
    let store = FakeFileStore::new().with(BUDGET_PATH, &budget(3));
    let text = metadata(&[("measured", true)]);
    let runner = runner(&store, "measured", 0, &text);

    // Act
    let outcome = EntryPoint::new(&runner, &store).run(&command(Action::Check, OutputFormat::Json));

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert!(outcome.stdout.starts_with(r#"{"stingy4rust":"check","version":1,"target":"x86_64-unknown-linux-gnu","verdict":"within""#), "{}", outcome.stdout);
    assert_eq!(outcome.stdout.lines().count(), 1);
}

#[test]
fn run_of_check_over_budget_exits_1() {
    // Arrange
    let store = FakeFileStore::new().with(BUDGET_PATH, &budget(2));
    let text = metadata(&[("measured", true)]);
    let runner = runner(&store, "measured", 0, &text);

    // Act
    let outcome =
        EntryPoint::new(&runner, &store).run(&command(Action::Check, OutputFormat::Table));

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert!(
        outcome
            .stdout
            .contains("1 within, 0 gains, 1 over, 0 unbudgeted, 0 missing"),
        "{}",
        outcome.stdout
    );
    assert!(outcome.stdout.contains("+1  over"), "{}", outcome.stdout);
}

#[test]
fn run_of_check_whose_run_failed_exits_2() {
    // Arrange
    let store = FakeFileStore::new().with(BUDGET_PATH, &budget(3));
    let text = metadata(&[("measured", true)]);
    let runner = runner(&store, "blocks-exhausted", 101, &text);

    // Act
    let outcome =
        EntryPoint::new(&runner, &store).run(&command(Action::Check, OutputFormat::Table));

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert_eq!(outcome.stdout, "");
    assert_eq!(
        outcome.stderr,
        "stingy4rust: profile three-blocks could not be measured: blocks-exhausted\n"
    );
}

#[test]
fn run_of_check_with_a_budget_that_cannot_be_read_exits_2() {
    // Arrange
    let store = FakeFileStore::new().with(BUDGET_PATH, "not json");
    let text = metadata(&[("measured", true)]);
    let runner = runner(&store, "measured", 0, &text);

    // Act
    let outcome =
        EntryPoint::new(&runner, &store).run(&command(Action::Check, OutputFormat::Table));

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert!(
        outcome
            .stderr
            .starts_with("stingy4rust: the budget /w/stingy4rust-budget.json could not be read: "),
        "{}",
        outcome.stderr
    );
}

#[test]
fn run_of_check_with_an_unbudgeted_figure_exits_1() {
    // Arrange
    let store = FakeFileStore::new();
    let text = metadata(&[("measured", true)]);
    let runner = runner(&store, "measured", 0, &text);

    // Act
    let outcome =
        EntryPoint::new(&runner, &store).run(&command(Action::Check, OutputFormat::Table));

    // Assert
    assert_eq!(outcome.exit_code, 1);
    assert!(
        outcome
            .stdout
            .contains("0 within, 0 gains, 0 over, 2 unbudgeted, 0 missing"),
        "{}",
        outcome.stdout
    );
}

#[test]
fn run_of_check_within_budget_exits_0() {
    // Arrange
    let store = FakeFileStore::new().with(BUDGET_PATH, &budget(3));
    let text = metadata(&[("measured", true)]);
    let runner = runner(&store, "measured", 0, &text);

    // Act
    let outcome =
        EntryPoint::new(&runner, &store).run(&command(Action::Check, OutputFormat::Table));

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert!(
        outcome
            .stdout
            .contains("2 within, 0 gains, 0 over, 0 unbudgeted, 0 missing"),
        "{}",
        outcome.stdout
    );
    assert_eq!(outcome.stderr, "");
}

#[test]
fn run_of_record_with_a_budget_that_cannot_be_written_exits_2() {
    // Arrange
    let store = FakeFileStore::new().refusing_writes();
    let text = metadata(&[("measured", true)]);
    let runner = runner(&store, "measured", 0, &text);

    // Act
    let outcome =
        EntryPoint::new(&runner, &store).run(&command(Action::Record, OutputFormat::Table));

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert_eq!(
        outcome.stderr,
        "stingy4rust: the budget /w/stingy4rust-budget.json could not be written: refused by the fake\n"
    );
}

#[test]
fn run_of_record_writes_the_budget_and_exits_0() {
    // Arrange
    let store = FakeFileStore::new().with(BUDGET_PATH, &budget(5));
    let text = metadata(&[("measured", true)]);
    let runner = runner(&store, "measured", 0, &text);

    // Act
    let outcome =
        EntryPoint::new(&runner, &store).run(&command(Action::Record, OutputFormat::Table));

    // Assert
    assert_eq!(outcome.exit_code, 0);
    let written = store.file(BUDGET_PATH).expect("the budget is written");
    let expected = BudgetFile::read(BUDGET_PATH, &budget(3)).expect("a budget");
    assert_eq!(BudgetFile::read(BUDGET_PATH, &written), Ok(expected));
    assert!(outcome.stdout.contains("-2  gain"), "{}", outcome.stdout);
    assert!(
        outcome
            .stdout
            .ends_with("recorded /w/stingy4rust-budget.json\n"),
        "{}",
        outcome.stdout
    );
}

#[test]
fn run_of_report_in_json_prints_the_report_as_written() {
    // Arrange
    let store = FakeFileStore::new();
    let text = metadata(&[("measured", true)]);
    let runner = runner(&store, "measured", 0, &text);

    // Act
    let outcome =
        EntryPoint::new(&runner, &store).run(&command(Action::Report, OutputFormat::Json));

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(outcome.stdout, report("measured", 3));
}

#[test]
fn run_of_report_reads_no_budget() {
    // Arrange
    let store = FakeFileStore::new().with(BUDGET_PATH, "not json");
    let text = metadata(&[("measured", true)]);
    let runner = runner(&store, "measured", 0, &text);

    // Act
    let outcome =
        EntryPoint::new(&runner, &store).run(&command(Action::Report, OutputFormat::Table));

    // Assert
    assert_eq!(outcome.exit_code, 0);
    assert_eq!(
        outcome.stdout,
        "profile three-blocks: measured\n\
         \x20 heap.allocations    3\n\
         \x20 peaks.run         192\n"
    );
}

#[test]
fn run_with_no_profile_target_exits_2() {
    // Arrange
    let store = FakeFileStore::new();
    let text = metadata(&[("tools", false)]);
    let runner = runner(&store, "measured", 0, &text);

    // Act
    let outcome =
        EntryPoint::new(&runner, &store).run(&command(Action::Check, OutputFormat::Table));

    // Assert
    assert_eq!(outcome.exit_code, 2);
    assert_eq!(
        outcome.stderr,
        "stingy4rust: no package of the workspace has a test target named stingy\n"
    );
}
