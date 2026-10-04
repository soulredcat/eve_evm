// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{FRAGMENT, assert_load_rejected, setup};

#[test]
fn fragment_count_duplicates_bytes_and_hard_physical_limit_are_bounded() {
    let duplicate = setup();
    duplicate.policy(&format!(
        "adapter_files = [\"{FRAGMENT}\", \"{FRAGMENT}\"]\n"
    ));
    assert_load_rejected(&duplicate);
    let excessive = setup();
    let files = std::iter::repeat_n(format!("\"{FRAGMENT}\""), 65)
        .collect::<Vec<_>>()
        .join(", ");
    excessive.policy(&format!("adapter_files = [{files}]\n"));
    assert_load_rejected(&excessive);
    let oversized = setup();
    oversized.write(
        FRAGMENT,
        &format!("# {}\nversion = 1\nadapters = []\n", "x".repeat(262_145)),
    );
    assert_load_rejected(&oversized);
    let hard_lines = setup();
    hard_lines.write(
        FRAGMENT,
        &format!(
            "version = 1\nadapters = []\n{}",
            "# Physical line.\n".repeat(599)
        ),
    );
    assert_load_rejected(&hard_lines);
}

#[test]
fn adapter_fragment_documents_keep_normal_decomposition_and_exception_requirements() {
    for lines in [201, 401, 600] {
        let fixture = setup();
        fixture.write(
            FRAGMENT,
            &format!(
                "version = 1\nadapters = []\n{}",
                "# Physical line.\n".repeat(lines - 2)
            ),
        );
        let report = fixture.assert_rejected();
        assert!(
            report
                .violations
                .iter()
                .any(|violation| violation.starts_with(FRAGMENT)
                    && (violation.contains("decomposition review")
                        || violation.contains("exception")))
        );
    }
}
