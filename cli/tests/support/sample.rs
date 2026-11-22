// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use std::path::Path;

pub const RUSTC: &str = "rustc 1.99.1 (8c2f8ad91 2026-08-27)\nbinary: rustc\ncommit-hash: 8c2f8ad91\nhost: x86_64-unknown-linux-gnu\nrelease: 1.99.1\n";
pub const TARGET: &str = "x86_64-unknown-linux-gnu";

pub fn budget(allocations: u64) -> String {
    format!(
        r#"{{"stingy4rust":"budget","version":1,"targets":{{"x86_64-unknown-linux-gnu":{{"three-blocks":{{"figures":{{"heap.allocations":{allocations},"peaks.run":192}}}}}}}}}}"#
    )
}

// Where the subcommand looks for the budget of the workspace at /w, joined as
// this platform joins a path.
pub fn budget_path() -> String {
    Path::new("/w")
        .join("stingy4rust-budget.json")
        .to_string_lossy()
        .into_owned()
}

pub fn metadata(packages: &[(&str, bool)]) -> String {
    let packages: Vec<String> = packages
        .iter()
        .map(|(name, profiled)| {
            let test = if *profiled {
                r#",{"name":"stingy","kind":["test"]}"#
            } else {
                ""
            };
            format!(r#"{{"name":"{name}","targets":[{{"name":"{name}","kind":["lib"]}}{test}]}}"#)
        })
        .collect();
    format!(
        r#"{{"packages":[{}],"workspace_root":"/w","target_directory":"/w/target","version":1}}"#,
        packages.join(",")
    )
}

pub fn report(status: &str, allocations: u64) -> String {
    format!(
        r#"{{"stingy4rust":"report","version":1,"profiles":[{{"name":"three-blocks","seed":2026,"status":"{status}","figures":{{"heap.allocations":{allocations},"peaks.run":192}},"bookkeeping":{{"block_capacity":8,"tracked_blocks_peak":3,"scope_capacity":4,"name_capacity":4,"untracked_frees":0,"ledger_bytes":4096}}}}]}}"#
    )
}

// Where the measured package's profile target writes its report, joined as
// this platform joins a path.
pub fn report_path() -> String {
    Path::new("/w/target")
        .join("stingy4rust")
        .join("measured.json")
        .to_string_lossy()
        .into_owned()
}
