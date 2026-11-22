// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::ledger::frame::Frame;
use stingy4rust::ledger::frame_stack::FrameStack;
use stingy4rust::ledger::max_depth::MAX_DEPTH;

const fn frame(slot: usize) -> Frame {
    Frame {
        slot,
        base: 0,
        high: 0,
    }
}

#[test]
fn clear_of_a_stack_holding_frames_empties_it() {
    // Arrange
    let mut stack = FrameStack::new();
    stack.push(frame(1));

    // Act
    stack.clear();

    // Assert
    assert_eq!(stack.current(), None);
    assert_eq!(stack.push(frame(2)), Some(1));
}

#[test]
fn current_names_the_slot_of_the_frame_on_top() {
    // Arrange
    let mut stack = FrameStack::new();
    stack.push(frame(1));
    stack.push(frame(2));

    // Act
    let current = stack.current();

    // Assert
    assert_eq!(current, Some(2));
}

#[test]
fn current_once_the_last_frame_is_popped_is_none() {
    // Arrange
    let mut stack = FrameStack::new();
    stack.push(frame(1));
    let popped = stack.pop();

    // Act
    let current = stack.current();

    // Assert
    assert_eq!(popped, Some(frame(1)));
    assert_eq!(current, None);
}

#[test]
fn pop_past_the_last_frame_returns_none() {
    // Arrange
    let mut stack = FrameStack::new();
    stack.push(frame(1));

    // Act
    let popped = [stack.pop(), stack.pop()];

    // Assert
    assert_eq!(popped, [Some(frame(1)), None]);
}

#[test]
fn pop_returns_the_frame_pushed_last() {
    // Arrange
    let mut stack = FrameStack::new();
    stack.push(frame(1));
    stack.push(frame(2));

    // Act
    let popped = stack.pop();

    // Assert
    assert_eq!(popped, Some(frame(2)));
    assert_eq!(stack.depth(), 1);
}

#[test]
fn push_past_the_maximum_depth_returns_none() {
    // Arrange
    let mut stack = FrameStack::new();
    for _ in 0..MAX_DEPTH {
        stack.push(frame(1));
    }

    // Act
    let depth = stack.push(frame(1));

    // Assert
    assert_eq!(depth, None);
    assert_eq!(stack.depth(), MAX_DEPTH);
}

#[test]
fn push_returns_the_depth_of_the_frame_it_opens() {
    // Arrange
    let mut stack = FrameStack::new();

    // Act
    let first = stack.push(frame(1));
    let second = stack.push(frame(2));

    // Assert
    assert_eq!(first, Some(1));
    assert_eq!(second, Some(2));
}

#[test]
fn raise_below_a_frame_s_high_keeps_its_high() {
    // Arrange
    let mut stack = FrameStack::new();
    stack.push(Frame {
        slot: 1,
        base: 0,
        high: 80,
    });

    // Act
    stack.raise(1, 50);

    // Assert
    assert_eq!(stack.pop().map(|frame| frame.high), Some(80));
}

#[test]
fn raise_lifts_the_high_of_every_open_frame_of_the_slot_and_no_other() {
    // Arrange
    let mut stack = FrameStack::new();
    stack.push(frame(1));
    stack.push(frame(2));
    stack.push(frame(1));

    // Act
    stack.raise(1, 50);

    // Assert
    assert_eq!(stack.pop().map(|frame| frame.high), Some(50));
    assert_eq!(stack.pop().map(|frame| frame.high), Some(0));
    assert_eq!(stack.pop().map(|frame| frame.high), Some(50));
}
