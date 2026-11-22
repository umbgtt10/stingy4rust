// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::run::support::recording_recorder::RecordingRecorder;
use stingy4rust::run::operation_guard::OperationGuard;

#[test]
fn drop_of_an_operation_guard_ends_its_operation() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let guard = OperationGuard::new(&RECORDER, "exchange");

    // Act
    drop(guard);

    // Assert
    assert_eq!(RECORDER.calls(), vec!["end operation exchange"]);
}
