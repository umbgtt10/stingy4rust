// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use crate::run::support::recording_recorder::RecordingRecorder;
use stingy4rust::run::scope_guard::ScopeGuard;

#[test]
fn drop_of_a_scope_guard_leaves_at_the_depth_it_was_given() {
    // Arrange
    static RECORDER: RecordingRecorder = RecordingRecorder::new(1);
    let guard = ScopeGuard::new(&RECORDER, 3);

    // Act
    drop(guard);

    // Assert
    assert_eq!(RECORDER.calls(), vec!["leave 3"]);
}
