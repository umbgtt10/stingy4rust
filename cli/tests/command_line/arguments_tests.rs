// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use clap::Parser;
use stingy4rust_cli::command_line::action::Action;
use stingy4rust_cli::command_line::arguments::Arguments;
use stingy4rust_cli::command_line::command::Command;
use stingy4rust_cli::command_line::output_format::OutputFormat;

fn parsed(arguments: &[&str]) -> Arguments {
    Arguments::try_parse_from(arguments).expect("the arguments parse")
}

#[test]
fn command_of_a_run_through_cargo_drops_the_subcommand_s_name() {
    // Arrange
    let arguments = parsed(&[
        "cargo-stingy4rust",
        "stingy4rust",
        "check",
        "--package",
        "measured",
    ]);

    // Act
    let command = arguments.command();

    // Assert
    assert_eq!(
        command,
        Command {
            action: Action::Check,
            package: Some(String::from("measured")),
            profile: None,
            target: None,
            format: OutputFormat::Table,
            budget: None,
        }
    );
}

#[test]
fn command_of_a_run_without_cargo_reads_the_action_first() {
    // Arrange
    let arguments = parsed(&["cargo-stingy4rust", "record"]);

    // Act
    let command = arguments.command();

    // Assert
    assert_eq!(command.action, Action::Record);
}

#[test]
fn command_with_every_option_carries_each() {
    // Arrange
    let arguments = parsed(&[
        "cargo-stingy4rust",
        "stingy4rust",
        "record",
        "--package",
        "measured",
        "--profile",
        "growing",
        "--target",
        "thumbv7em-none-eabihf",
        "--format",
        "json",
        "--budget",
        "/tmp/budget.json",
    ]);

    // Act
    let command = arguments.command();

    // Assert
    assert_eq!(
        command,
        Command {
            action: Action::Record,
            package: Some(String::from("measured")),
            profile: Some(String::from("growing")),
            target: Some(String::from("thumbv7em-none-eabihf")),
            format: OutputFormat::Json,
            budget: Some(String::from("/tmp/budget.json")),
        }
    );
}
