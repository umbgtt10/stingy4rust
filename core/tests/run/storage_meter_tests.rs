// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::run::support::recording_recorder::RecordingRecorder;
use stingy4rust::figures::scope_name::ScopeName;
use stingy4rust::run::storage_meter::StorageMeter;

const JOURNAL: ScopeName = ScopeName {
    name: "journal",
    number: None,
};

#[test]
fn at_rest_records_what_the_store_holds() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let meter = StorageMeter::new(&RECORDER, JOURNAL);

    // Act
    meter.at_rest(150);

    // Assert
    assert_eq!(RECORDER.calls(), vec!["at rest journal 150"]);
}

#[test]
fn on_disk_records_the_store_s_files_size() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let meter = StorageMeter::new(&RECORDER, JOURNAL);

    // Act
    meter.on_disk(4096);

    // Assert
    assert_eq!(RECORDER.calls(), vec!["on disk journal 4096"]);
}

#[test]
fn transaction_records_one_for_the_store() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let meter = StorageMeter::new(&RECORDER, JOURNAL);

    // Act
    meter.transaction();

    // Assert
    assert_eq!(RECORDER.calls(), vec!["transaction journal"]);
}

#[test]
fn written_records_the_bytes_of_one_write() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let meter = StorageMeter::new(&RECORDER, JOURNAL);

    // Act
    meter.written(50);

    // Assert
    assert_eq!(RECORDER.calls(), vec!["written journal 50"]);
}
