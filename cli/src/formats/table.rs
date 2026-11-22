// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust::budget::check_outcome::CheckOutcome;
use stingy4rust::budget::figure_verdict::FigureVerdict;
use stingy4rust::budget::verdict::Verdict;

use crate::formats::reported_profile::ReportedProfile;
use crate::formats::verdict_name::VerdictName;

const HEADER: [&str; 5] = ["path", "budget", "measured", "difference", "verdict"];
const NONE: &str = "-";

// What a person reads: a report's figures, a profile a block; a check's, one
// row a figure -- path, budget, measured, difference, verdict -- then the
// count of each verdict, and a hint when a gain is there to be recorded.
pub struct Table;

impl Table {
    #[must_use]
    pub fn check(outcome: &CheckOutcome) -> String {
        let blocks: Vec<String> = outcome
            .profiles
            .iter()
            .map(|profile| Self::checked_block(&profile.name, &profile.figures))
            .collect();
        let verdicts: Vec<Verdict> = outcome
            .profiles
            .iter()
            .flat_map(|profile| &profile.figures)
            .map(|figure| figure.verdict)
            .collect();
        format!(
            "target {}\n\n{}\n{}",
            outcome.target,
            blocks.join("\n"),
            Self::summary(&verdicts)
        )
    }

    #[must_use]
    pub fn report(profiles: &[ReportedProfile]) -> String {
        let blocks: Vec<String> = profiles
            .iter()
            .map(|profile| {
                let rows: Vec<[String; 2]> = profile
                    .measured
                    .figures
                    .iter()
                    .map(|figure| [figure.path.clone(), figure.value.to_string()])
                    .collect();
                format!(
                    "profile {}: {}\n{}",
                    profile.measured.name,
                    profile.status,
                    Self::rows(&rows, &[false, true])
                )
            })
            .collect();
        blocks.join("\n")
    }

    fn cell(value: Option<u64>) -> String {
        value.map_or_else(|| String::from(NONE), |value| value.to_string())
    }

    fn checked_block(name: &str, figures: &[FigureVerdict]) -> String {
        let mut rows = vec![HEADER.map(String::from)];
        rows.extend(figures.iter().map(|figure| {
            [
                figure.path.clone(),
                Self::cell(figure.budget),
                Self::cell(figure.measured),
                Self::difference(figure.budget, figure.measured),
                String::from(VerdictName::of(figure.verdict)),
            ]
        }));
        format!(
            "profile {name}\n{}",
            Self::rows(&rows, &[false, true, true, true, false])
        )
    }

    fn difference(budget: Option<u64>, measured: Option<u64>) -> String {
        match (budget, measured) {
            (Some(budget), Some(measured)) if measured > budget => {
                format!("+{}", measured - budget)
            }
            (Some(budget), Some(measured)) if measured < budget => {
                format!("-{}", budget - measured)
            }
            (Some(_), Some(_)) => String::from("0"),
            _ => String::from(NONE),
        }
    }

    // Rows of cells in columns as wide as their widest cell, two spaces apart,
    // each row indented by two; a column marked right-aligned is.
    fn rows<const COLUMNS: usize>(rows: &[[String; COLUMNS]], right: &[bool; COLUMNS]) -> String {
        let widths: Vec<usize> = (0..COLUMNS)
            .map(|column| rows.iter().map(|row| row[column].len()).max().unwrap_or(0))
            .collect();
        rows.iter()
            .map(|row| {
                let cells: Vec<String> = (0..COLUMNS)
                    .map(|column| {
                        let width = widths[column];
                        let cell = &row[column];
                        if right[column] {
                            format!("{cell:>width$}")
                        } else if column + 1 == COLUMNS {
                            cell.clone()
                        } else {
                            format!("{cell:<width$}")
                        }
                    })
                    .collect();
                format!("  {}\n", cells.join("  "))
            })
            .collect::<Vec<String>>()
            .concat()
    }

    fn summary(verdicts: &[Verdict]) -> String {
        let count = |wanted: Verdict| {
            verdicts
                .iter()
                .filter(|verdict| **verdict == wanted)
                .count()
        };
        let gains = count(Verdict::Gain);
        let mut text = format!(
            "{} within, {gains} {}, {} over, {} unbudgeted, {} missing\n",
            count(Verdict::Within),
            if gains == 1 { "gain" } else { "gains" },
            count(Verdict::Over),
            count(Verdict::Unbudgeted),
            count(Verdict::Missing)
        );
        if gains > 0 {
            text.push_str("a gain is kept by recording it: cargo stingy4rust record\n");
        }
        text
    }
}
