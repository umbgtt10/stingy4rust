// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::alloc::System;

use crate::stingy::Stingy;

// The host's counting allocator over the system's: 128 scope names, 262,144
// live blocks, 256 kinds, stores and operations -- some 8 MiB of static
// memory, reported in the bookkeeping. A project that needs more names its own
// `Stingy<System, S, B, N>`.
pub type HostStingy = Stingy<System, 128, 262_144, 256>;
