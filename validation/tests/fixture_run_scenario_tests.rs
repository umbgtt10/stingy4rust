// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use validation::fixture_run::FixtureRun;
use validation::fixture_run_scenario::FixtureRunScenario;

fn assert_figures(run: &FixtureRun, profile: &str, expected: &[(&str, u64)]) {
    for (path, value) in expected {
        assert_eq!(
            run.figure(profile, path),
            Some(*value),
            "{profile} {path} in {}\n{}",
            run.report(),
            run.log()
        );
    }
}

#[test]
fn run_of_freed_elsewhere_credits_the_scope_that_allocated() {
    // Arrange & Act
    let run = FixtureRunScenario::run();

    // Assert
    assert_figures(
        run,
        "freed-elsewhere",
        &[
            ("scopes.producer.allocations", 1),
            ("scopes.producer.bytes_allocated", 100),
            ("scopes.producer.bytes_freed", 100),
            ("scopes.producer.live_at_end", 0),
            ("scopes.consumer.allocations", 0),
            ("scopes.consumer.bytes_freed", 0),
            ("scopes.consumer.entries", 1),
        ],
    );
}

#[test]
fn run_of_growing_reports_four_reallocations_of_a_vec() {
    // Arrange & Act
    let run = FixtureRunScenario::run();

    // Assert
    assert_figures(
        run,
        "growing",
        &[
            ("scopes.grower.allocations", 1),
            ("scopes.grower.reallocations", 4),
            ("scopes.grower.bytes_allocated", 248),
            ("scopes.grower.bytes_freed", 248),
            ("scopes.grower.peak", 128),
        ],
    );
}

#[test]
fn run_of_nested_reports_each_scope_and_the_run_s_peak() {
    // Arrange & Act
    let run = FixtureRunScenario::run();

    // Assert
    assert_figures(
        run,
        "nested",
        &[
            ("scopes.outer.allocations", 1),
            ("scopes.outer.bytes_allocated", 10),
            ("scopes.inner.allocations", 1),
            ("scopes.inner.bytes_allocated", 20),
            ("peaks.run", 30),
        ],
    );
}

#[test]
fn run_of_outside_charges_the_outside_scopes_slot() {
    // Arrange & Act
    let run = FixtureRunScenario::run();

    // Assert
    assert_figures(
        run,
        "outside",
        &[
            ("scopes.(outside scopes).allocations", 1),
            ("scopes.(outside scopes).bytes_allocated", 32),
            ("scopes.(outside scopes).bytes_freed", 32),
        ],
    );
}

#[test]
fn run_of_three_blocks_reports_192_bytes_in_the_worker_scope() {
    // Arrange & Act
    let run = FixtureRunScenario::run();

    // Assert
    assert_figures(
        run,
        "three-blocks",
        &[
            ("scopes.worker.allocations", 3),
            ("scopes.worker.reallocations", 0),
            ("scopes.worker.bytes_allocated", 192),
            ("scopes.worker.bytes_freed", 192),
            ("scopes.worker.live_at_end", 0),
            ("scopes.worker.peak", 192),
            ("scopes.worker.largest_entry_rise", 192),
            ("scopes.worker.entries", 1),
            ("heap.allocations", 3),
            ("peaks.run", 192),
        ],
    );
}

#[test]
fn run_of_wire_and_storage_reports_the_documented_figures() {
    // Arrange & Act
    let run = FixtureRunScenario::run();

    // Assert
    assert_figures(
        run,
        "wire-and-storage",
        &[
            ("wire.ping.messages", 2),
            ("wire.ping.bytes", 20),
            ("wire.pong.messages", 2),
            ("wire.pong.bytes", 200),
            ("operations.exchange.count", 2),
            ("operations.exchange.messages", 4),
            ("operations.exchange.bytes", 220),
            ("storage.journal.bytes_written", 150),
            ("storage.journal.writes", 3),
            ("storage.journal.transactions", 2),
            ("storage.journal.bytes_at_rest", 150),
            ("storage.journal.bytes_on_disk", 4096),
        ],
    );
}

#[test]
fn run_twice_writes_the_same_report() {
    // Arrange & Act
    let reports = FixtureRunScenario::run_twice();

    // Assert
    assert!(
        reports.first.contains("\"stingy4rust\": \"report\""),
        "{}",
        reports.first
    );
    assert_eq!(reports.first, reports.second);
}
