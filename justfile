# Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
# Licensed under the MIT License
# SPDX-License-Identifier: MIT

# just looks for a POSIX `sh` on Windows and there is not reliably one on PATH:
# Git for Windows ships sh.exe without putting it there, and a resolvable
# bash.exe may belong to WSL, a separate toolchain. PowerShell is the one shell
# guaranteed present. Only recipe bodies are shell-interpreted, so this affects
# Windows alone; the Linux and macOS runners use just's default `sh`.
set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

# Set by just itself rather than per recipe line, so it reaches whichever shell
# runs the body without needing sh and PowerShell syntax for the same thing.
export RUSTFLAGS := "-D warnings"

# CI fails on drift instead of silently rewriting files nobody is there to
# review; a local run still formats in place.
fmt_mode := if env('CI', '') != '' { '--check' } else { '' }

default:
    @just --list

# Every commit message gains core's grip and braintax, scored as staged, by the
# commit-msg hook in .githooks/. Once a clone.
hooks:
    git config core.hooksPath .githooks

# Formatting, clippy, the core built for a microcontroller -- it is `no_std`,
# and a host build proves nothing about that -- and the tests. Cargo built-ins
# only, so it works on a fresh checkout with none of the house tools installed.
stage1:
    cargo fmt {{fmt_mode}}
    cargo clippy --workspace --all-targets -- -D warnings
    cargo check -p stingy4rust-core --no-default-features --target thumbv7em-none-eabihf
    cargo test --workspace

# The technology-free core, unsafe confined to the allocators, house rules,
# dependencies, duplication, CRAP, mirrored tests, file risk, the licences and
# advisories of the dependencies and the tool's own budget, run in that order.
# The core's gate runs first because its fix moves code between crates, and the
# unsafe gate, as cheap, second; stern runs third because its corrections are
# renames, file moves and directory splits: a layout it is about to reject is
# one the others would have measured for nothing. The own budget runs last
# because it builds the subcommand and the fixture in release.
stage2:
    cargo xtask stage2

# ripe4rust's plan: which moves of the toolchain and the dependencies are ripe.
ripe:
    cargo ripe4rust plan
