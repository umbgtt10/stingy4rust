// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use alloc::string::String;

use crate::figures::flat_figures::FlatFigures;
use crate::ledger::bookkeeping::Bookkeeping;
use crate::ledger::run_status::RunStatus;
use crate::report::json_writer::JsonWriter;
use crate::report::profile_report::ProfileReport;
use crate::report::report::Report;

const VERSION: u64 = 1;

// A report as the design's section 7.2 writes it: every profile in the order
// run, its status in kebab case, its figures in the order of section 7.1 and
// its bookkeeping, keys in a fixed order, so the same runs write the same
// bytes. The core's own writer: no serialisation crate in the core.
pub struct ReportJson;

impl ReportJson {
    #[must_use]
    pub fn write(report: &Report) -> String {
        let writer = JsonWriter::new()
            .begin_object()
            .key("stingy4rust")
            .string("report")
            .key("version")
            .number(VERSION)
            .key("profiles")
            .begin_array();
        let mut text = report
            .profiles
            .iter()
            .fold(writer, Self::profile)
            .end_array()
            .end_object()
            .into_text();
        text.push('\n');
        text
    }

    fn bookkeeping(writer: JsonWriter, bookkeeping: Bookkeeping) -> JsonWriter {
        writer
            .key("bookkeeping")
            .begin_object()
            .key("block_capacity")
            .number(bookkeeping.block_capacity)
            .key("tracked_blocks_peak")
            .number(bookkeeping.tracked_blocks_peak)
            .key("scope_capacity")
            .number(bookkeeping.scope_capacity)
            .key("name_capacity")
            .number(bookkeeping.name_capacity)
            .key("untracked_frees")
            .number(bookkeeping.untracked_frees)
            .key("ledger_bytes")
            .number(bookkeeping.ledger_bytes)
            .end_object()
    }

    fn profile(writer: JsonWriter, profile: &ProfileReport) -> JsonWriter {
        let writer = writer
            .begin_object()
            .key("name")
            .string(profile.name)
            .key("seed")
            .number(profile.seed)
            .key("status")
            .string(Self::status(profile.tally.status))
            .key("figures")
            .begin_object();
        let writer = FlatFigures::of(&profile.tally)
            .iter()
            .fold(writer, |writer, figure| {
                writer.key(&figure.path).number(figure.value)
            })
            .end_object();
        Self::bookkeeping(writer, profile.tally.bookkeeping).end_object()
    }

    const fn status(status: RunStatus) -> &'static str {
        match status {
            RunStatus::Measured => "measured",
            RunStatus::ScopesExhausted => "scopes-exhausted",
            RunStatus::BlocksExhausted => "blocks-exhausted",
            RunStatus::NamesExhausted => "names-exhausted",
            RunStatus::DepthExhausted => "depth-exhausted",
            RunStatus::ForeignThread => "foreign-thread",
            RunStatus::ScopeLeftOutOfOrder => "scope-left-out-of-order",
            RunStatus::NameWithDot => "name-with-dot",
        }
    }
}
