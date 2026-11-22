// Copyright 2026 Umberto Gotti <umberto.gotti@umbertogotti.dev>
// Licensed under the MIT License
// SPDX-License-Identifier: MIT

use stingy4rust_cli::formats::budget_file::BudgetFile;
use stingy4rust_cli::unable::Unable;

const BUDGET: &str = "stingy4rust-budget.json";
const UNSORTED: &str = r#"{
  "stingy4rust": "budget",
  "version": 1,
  "targets": {
    "x86_64-unknown-linux-gnu": {
      "growing": {
        "figures": {
          "peaks.run": 128,
          "heap.allocations": 1
        }
      },
      "freed-elsewhere": {
        "figures": {
          "storage.log.bytes_on_disk": 8192
        },
        "tolerances": {
          "storage.log.bytes_on_disk": {
            "absolute": 4096,
            "relative": 0.5
          }
        }
      }
    }
  }
}
"#;

#[test]
fn read_of_a_budget_of_an_unknown_version_fails() {
    // Arrange
    let text = UNSORTED.replace("\"version\": 1", "\"version\": 7");

    // Act
    let budget = BudgetFile::read(BUDGET, &text);

    // Assert
    assert_eq!(
        budget,
        Err(Unable::UnreadableBudget {
            path: String::from(BUDGET),
            reason: String::from("it is version 7; this stingy4rust reads version 1"),
        })
    );
}

#[test]
fn read_of_text_that_is_not_a_budget_fails() {
    // Arrange
    let text = "{\"stingy4rust\": \"report\", \"version\": 1, \"targets\": {}}";

    // Act
    let budget = BudgetFile::read(BUDGET, text);

    // Assert
    assert_eq!(
        budget,
        Err(Unable::UnreadableBudget {
            path: String::from(BUDGET),
            reason: String::from("it is a report, not a budget"),
        })
    );
    assert!(matches!(
        BudgetFile::read(BUDGET, "[]"),
        Err(Unable::UnreadableBudget { .. })
    ));
}

#[test]
fn read_of_what_write_wrote_is_the_same_budget() {
    // Arrange
    let budget = BudgetFile::read(BUDGET, UNSORTED).expect("a budget");

    // Act
    let again = BudgetFile::read(BUDGET, &BudgetFile::write(&budget));

    // Assert
    assert_eq!(budget.targets["x86_64-unknown-linux-gnu"].len(), 2);
    assert_eq!(again, Ok(budget));
}

#[test]
fn write_of_a_budget_sorts_its_keys() {
    // Arrange
    let budget = BudgetFile::read(BUDGET, UNSORTED).expect("a budget");

    // Act
    let text = BudgetFile::write(&budget);

    // Assert
    assert_eq!(
        text,
        r#"{
  "stingy4rust": "budget",
  "version": 1,
  "targets": {
    "x86_64-unknown-linux-gnu": {
      "freed-elsewhere": {
        "figures": {
          "storage.log.bytes_on_disk": 8192
        },
        "tolerances": {
          "storage.log.bytes_on_disk": {
            "absolute": 4096,
            "relative": 0.5
          }
        }
      },
      "growing": {
        "figures": {
          "heap.allocations": 1,
          "peaks.run": 128
        }
      }
    }
  }
}
"#
    );
}
