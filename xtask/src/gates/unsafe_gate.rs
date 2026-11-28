// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::{Path, PathBuf};

use syn::{File, parse_file};

use crate::boundary::unsafe_rule::UnsafeRule;
use crate::boundary::unsafe_use_collector::UnsafeUseCollector;
use serge4rust::gates::boundary::source_files::SourceFiles;
use serge4rust::gates::gate::Gate;
use serge4rust::gates::gate_failure::GateFailure;

// `unsafe` confined to the files allowed to hold it: every other `.rs` file
// beneath the rule's directories is parsed, and each one that holds any kind of
// `unsafe` is named, all at once, with what it holds.
pub struct UnsafeGate<'a> {
    sources: &'a dyn SourceFiles,
    rule: UnsafeRule,
}

impl<'a> UnsafeGate<'a> {
    pub fn new(sources: &'a dyn SourceFiles, rule: UnsafeRule) -> Self {
        Self { sources, rule }
    }

    fn files(&self) -> Result<Vec<PathBuf>, GateFailure> {
        let mut files = Vec::new();
        for dir in &self.rule.source_dirs {
            files.extend(self.sources.rust_files(dir)?);
        }
        if files.is_empty() {
            return Err(GateFailure::Own {
                reason: String::from("no source file was scanned for unsafe"),
            });
        }
        Ok(files)
    }

    fn finding(&self, file: &Path) -> Result<Option<String>, GateFailure> {
        let uses = UnsafeUseCollector::collect(&self.parsed(file)?);
        if uses.is_empty() {
            return Ok(None);
        }
        let allowed: Vec<String> = self
            .rule
            .allowed_files
            .iter()
            .map(|allowed| allowed.display().to_string())
            .collect();
        Ok(Some(format!(
            "{} uses {}, which only {} may",
            file.display(),
            uses.join(", "),
            allowed.join(" and ")
        )))
    }

    fn parsed(&self, file: &Path) -> Result<File, GateFailure> {
        let source = self.sources.read(file)?;
        parse_file(&source).map_err(|error| GateFailure::Unparsable {
            file: file.display().to_string(),
            reason: error.to_string(),
        })
    }
}

impl Gate for UnsafeGate<'_> {
    fn label(&self) -> String {
        String::from("Unsafe confined")
    }

    fn run(&self) -> Result<(), GateFailure> {
        let mut findings = Vec::new();
        for file in self.files()? {
            if self.rule.allowed_files.contains(&file) {
                continue;
            }
            findings.extend(self.finding(&file)?);
        }
        if findings.is_empty() {
            Ok(())
        } else {
            Err(GateFailure::Breached { findings })
        }
    }
}
