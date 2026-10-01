// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Fixture;

#[test]
fn test_languages_report_external_compiler_requirement_without_claiming_ast_coverage() {
    let fixture = Fixture::new();
    for (path, source) in [
        (
            "tests/acceptance/client/send_transfer.ts",
            "export function sendTransfer(): void {}\n",
        ),
        (
            "tests/acceptance/contracts/Token.sol",
            "pragma solidity 0.8.37;\ncontract Token {}\n",
        ),
    ] {
        fixture.write(path, source);
    }
    for file in fixture
        .assert_pass()
        .files
        .into_iter()
        .filter(|file| file.path.ends_with(".ts") || file.path.ends_with(".sol"))
    {
        assert_eq!(file.kind, "test");
        assert_eq!(file.syntax_coverage, "TEST_SOURCE_REQUIRES_BULK_COMPILER");
    }
}

#[test]
fn production_languages_remain_rejected_without_equivalent_behavior_parser() {
    for path in [
        "public/src/send_transfer.ts",
        "validator/src/Token.sol",
        "public/src/tests/send_transfer.ts",
        "validator/components/execution/src/tests/Token.sol",
    ] {
        let fixture = Fixture::new();
        fixture.write(path, "// Unsupported production source.\n");
        let report = fixture.assert_rejected();
        let file = report.files.iter().find(|file| file.path == path).unwrap();
        assert_eq!(file.kind, "unsupported");
        assert_eq!(file.syntax_coverage, "NOT_IMPLEMENTED");
    }
}

#[test]
fn external_test_sources_preserve_every_physical_size_boundary() {
    for extension in ["ts", "sol"] {
        for lines in [200, 201, 400, 401, 600, 601] {
            for terminated in [true, false] {
                let fixture = Fixture::new();
                let path = format!("tests/acceptance/fixtures/size_boundary.{extension}");
                let mut source = if extension == "ts" {
                    "export function fixture(): void {}\n".to_owned()
                } else {
                    "pragma solidity 0.8.37; contract Fixture {}\n".to_owned()
                };
                source.push_str(&"// Count complete physical source.\n".repeat(lines - 1));
                if !terminated {
                    source.pop();
                }
                fixture.write(&path, &source);
                assert_eq!(fixture.check().violations.is_empty(), lines == 200);
                let review = match lines {
                    201 | 400 => format!(
                        "[[size_reviews]]\npath = \"{path}\"\nlines = {lines}\nreason = \"Physical boundary test fixture.\"\nreviewer = \"test-reviewer\"\n"
                    ),
                    401 | 600 | 601 => format!(
                        "[[exceptions]]\npath = \"{path}\"\nlines = {lines}\nreason = \"Physical boundary test fixture.\"\nreviewer = \"test-reviewer\"\nsplit_task = \"Split fixture in B1.\"\nrelated_tests = [\"external_test_sources_preserve_every_physical_size_boundary\"]\nexpires_bulk = 1\n"
                    ),
                    _ => String::new(),
                };
                fixture.policy(&review);
                let report = fixture.check();
                assert_eq!(report.violations.is_empty(), lines <= 600);
                assert_eq!(
                    report
                        .files
                        .iter()
                        .find(|file| file.path == path)
                        .unwrap()
                        .physical_lines,
                    lines
                );
            }
        }
    }
}
