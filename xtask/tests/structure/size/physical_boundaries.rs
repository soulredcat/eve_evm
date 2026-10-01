// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{Fixture, plain_policy, source_with_lines};
use xtask::structure::inspection::{
    count_physical_lines::count_physical_lines, review_size::review_size,
};

#[test]
fn t_l03_physical_line_count_includes_blanks_comments_and_unterminated_last_line() {
    for lines in [200, 201, 400, 401, 600, 601] {
        for terminated in [true, false] {
            let source = source_with_lines(lines, terminated);
            assert_eq!(source.ends_with('\n'), terminated);
            assert_eq!(count_physical_lines(&source), lines);
            assert!(source.contains("\n\n"));
        }
    }
    for (source, count) in [("", 0), ("\n", 1), ("a", 1), ("a\n", 1), ("a\r\nb", 2)] {
        assert_eq!(count_physical_lines(source), count);
    }
}

#[test]
fn t_l03_exact_boundary_files_require_the_correct_review_and_exception() {
    for lines in [200, 201, 400, 401, 600, 601] {
        for terminated in [true, false] {
            let fixture = Fixture::new();
            let path = "validator/src/encode_height.rs";
            fixture.write(path, &source_with_lines(lines, terminated));
            let initial = fixture.check();
            assert_eq!(initial.violations.is_empty(), lines == 200, "{lines} lines");
            let registration = match lines {
                201 | 400 => format!(
                    "[[size_reviews]]\npath = \"{path}\"\nlines = {lines}\nreason = \"One encoding operation; blank and comment fixture exercises physical size.\"\nreviewer = \"test-reviewer\"\n"
                ),
                401 | 600 | 601 => format!(
                    "[[exceptions]]\npath = \"{path}\"\nlines = {lines}\nreason = \"Bounded size regression fixture.\"\nreviewer = \"test-reviewer\"\nsplit_task = \"Split fixture by encoding scenario in B1.\"\nexpires_bulk = 1\nrelated_tests = [\"T-L03\"]\n"
                ),
                _ => String::new(),
            };
            fixture.policy(&registration);
            let reviewed = fixture.check();
            let file = reviewed
                .files
                .iter()
                .find(|file| file.path == path)
                .unwrap();
            assert_eq!(file.physical_lines, lines);
            assert_eq!(
                reviewed.violations.is_empty(),
                lines <= 600,
                "{lines}: {:?}",
                reviewed.violations
            );
            assert_eq!(!reviewed.warnings.is_empty(), (201..=600).contains(&lines));
        }
    }
}

#[test]
fn t_l03_test_document_and_behavior_files_share_the_hard_ceiling() {
    for (path, source) in [
        (
            "validator/tests/encode_height.rs",
            source_with_lines(601, false),
        ),
        (
            "docs/encoding.md",
            "Reviewed encoding contract.\n".repeat(601),
        ),
        (
            "config/encoding.toml",
            "# Maintained configuration comment.\n".repeat(601),
        ),
    ] {
        let fixture = Fixture::new();
        fixture.write(path, &source);
        let report = fixture.assert_rejected();
        assert!(
            report
                .violations
                .iter()
                .any(|message| message.contains("600-line"))
        );
    }
}

#[test]
fn reviewed_line_count_must_match_the_complete_current_file() {
    let policy = plain_policy();
    for lines in [201, 400, 401, 600] {
        let (warnings, violations) = review_size("validator/src/encode_height.rs", lines, &policy);
        assert!(warnings.is_empty());
        assert_eq!(violations.len(), 1);
    }
    let fixture = Fixture::new();
    fixture.write(
        "validator/src/encode_height.rs",
        &source_with_lines(201, true),
    );
    fixture.policy("[[size_reviews]]\npath = \"validator/src/encode_height.rs\"\nlines = 200\nreason = \"Reviewed earlier version.\"\nreviewer = \"test-reviewer\"\n");
    fixture.assert_rejected();
}
