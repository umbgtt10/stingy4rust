// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use serde::Deserialize;
use serde_json::from_str;
use stingy4rust::budget::measured_profile::MeasuredProfile;

use crate::formats::figure_list::FigureList;
use crate::formats::reported_profile::ReportedProfile;
use crate::unable::Unable;

const VERSION: u64 = 1;

#[derive(Deserialize)]
struct ReportDocument {
    pub stingy4rust: String,
    pub version: u64,
    pub profiles: Vec<ReportProfileRecord>,
}

#[derive(Deserialize)]
struct ReportProfileRecord {
    pub name: String,
    pub status: String,
    pub figures: FigureList,
}

// A report as a profile target writes it, read back: each profile's status
// and its figures in the order written. A document of another kind or of
// another version is refused.
pub struct ReportFile;

impl ReportFile {
    pub fn read(text: &str) -> Result<Vec<ReportedProfile>, Unable> {
        let document: ReportDocument =
            from_str(text).map_err(|error| Unable::UnreadableReport(error.to_string()))?;
        if document.stingy4rust != "report" {
            return Err(Unable::UnreadableReport(format!(
                "it is a {}, not a report",
                document.stingy4rust
            )));
        }
        if document.version != VERSION {
            return Err(Unable::UnreadableReport(format!(
                "it is version {}; this stingy4rust reads version {VERSION}",
                document.version
            )));
        }
        Ok(document
            .profiles
            .into_iter()
            .map(|profile| ReportedProfile {
                status: profile.status,
                measured: MeasuredProfile {
                    name: profile.name,
                    figures: profile.figures.into_figures(),
                },
            })
            .collect())
    }
}
