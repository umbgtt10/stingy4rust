// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::support::recording_recorder::RecordingRecorder;
use stingy4rust::run::run::Run;

#[test]
fn name_of_a_run_is_its_profile_s() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let run = Run::new("three-blocks", 2026, &RECORDER);

    // Act
    let name = run.name();

    // Assert
    assert_eq!(name, "three-blocks");
}

#[test]
fn operation_of_a_run_begins_and_its_guard_ends_it() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let run = Run::new("profile", 2026, &RECORDER);

    // Act
    let guard = run.operation("exchange");
    drop(guard);

    // Assert
    assert_eq!(
        RECORDER.calls(),
        vec!["begin operation exchange", "end operation exchange"]
    );
}

#[test]
fn scope_numbered_names_the_scope_with_its_number() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(4);
    let run = Run::new("profile", 2026, &RECORDER);

    // Act
    let guard = run.scope_numbered("node", 3);
    drop(guard);

    // Assert
    assert_eq!(RECORDER.calls(), vec!["enter node 3", "leave 4"]);
}

#[test]
fn scope_of_a_run_enters_and_its_guard_leaves_at_the_depth_entered() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(7);
    let run = Run::new("profile", 2026, &RECORDER);

    // Act
    let guard = run.scope("worker");
    drop(guard);

    // Assert
    assert_eq!(RECORDER.calls(), vec!["enter worker", "leave 7"]);
}

#[test]
fn seed_of_a_run_is_its_profile_s() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let run = Run::new("three-blocks", 2026, &RECORDER);

    // Act
    let seed = run.seed();

    // Assert
    assert_eq!(seed, 2026);
}

#[test]
fn storage_numbered_of_a_run_names_its_store_with_its_number() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let run = Run::new("profile", 2026, &RECORDER);

    // Act
    run.storage_numbered("log", 2).transaction();

    // Assert
    assert_eq!(RECORDER.calls(), vec!["transaction log 2"]);
}

#[test]
fn storage_of_a_run_names_its_store() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let run = Run::new("profile", 2026, &RECORDER);

    // Act
    run.storage("journal").written(50);

    // Assert
    assert_eq!(RECORDER.calls(), vec!["written journal 50"]);
}

#[test]
fn wire_of_a_run_records_through_the_run_s_recorder() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let run = Run::new("profile", 2026, &RECORDER);

    // Act
    run.wire().sent("ping", 10);

    // Assert
    assert_eq!(RECORDER.calls(), vec!["sent ping 10"]);
}
