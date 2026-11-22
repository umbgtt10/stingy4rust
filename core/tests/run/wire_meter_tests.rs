// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::run::support::recording_recorder::RecordingRecorder;
use stingy4rust::run::wire_meter::WireMeter;

#[test]
fn sent_records_the_kind_and_its_bytes() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let meter = WireMeter::new(&RECORDER);

    // Act
    meter.sent("pong", 100);

    // Assert
    assert_eq!(RECORDER.calls(), vec!["sent pong 100"]);
}
