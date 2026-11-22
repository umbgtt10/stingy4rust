// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::thread;
use stingy4rust_host::thread_mark::ThreadMark;

#[test]
fn current_on_another_thread_differs() {
    // Arrange
    let here = ThreadMark::current();

    // Act
    let there = thread::spawn(ThreadMark::current)
        .join()
        .expect("the thread runs");

    // Assert
    assert_ne!(here, there);
}

#[test]
fn current_on_one_thread_is_the_same_twice() {
    // Arrange
    let first = ThreadMark::current();

    // Act
    let second = ThreadMark::current();

    // Assert
    assert_eq!(first, second);
    assert_ne!(first, 0);
}
