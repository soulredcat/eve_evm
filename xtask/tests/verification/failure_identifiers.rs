// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use xtask::verification::commands::summarize_registered_test_failures::summarize_registered_test_failures;

#[test]
fn failure_identifiers_are_registered_exact_deduplicated_and_sorted() {
    let registered = vec![
        "cases::beta".into(),
        "cases::alpha".into(),
        "cases::passed".into(),
    ];
    let output = "test cases::beta ... FAILED\ntest cases::passed ... ok\n\
        failures:\n    cases::beta\n    cases::alpha\n\
        test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out;\n";
    assert_eq!(
        summarize_registered_test_failures(output, &registered),
        "cases::alpha, cases::beta"
    );
}

#[test]
fn failure_identifiers_never_echo_unknown_messages_paths_or_control_characters() {
    let registered = vec![
        "cases::valid".into(),
        "private-key=secret".into(),
        "cases::bad\n/personal/path".into(),
        "cases::escape\x1b[31m".into(),
    ];
    let output = "test unknown::secret ... FAILED\n\
        test cases::valid ... FAILED private-key=secret\n\
        failures:\n    /personal/path\n    private-key=secret\n    cases::escape\x1b[31m\n\
        test result: FAILED. 0 passed; 1 failed;\n";
    assert_eq!(
        summarize_registered_test_failures(output, &registered),
        "NO_REGISTERED_FAILURE_ID"
    );
    assert_eq!(
        summarize_registered_test_failures("cases::valid\n", &registered),
        "NO_REGISTERED_FAILURE_ID"
    );
}

#[test]
fn failure_identifier_summary_bounds_count_and_identifier_length() {
    let mut registered: Vec<String> = (0..40)
        .map(|index| format!("cases::case_{index:02}"))
        .collect();
    registered.push(format!("cases::{}", "a".repeat(513)));
    let output = registered
        .iter()
        .map(|name| format!("test {name} ... FAILED\n"))
        .collect::<String>();
    let summary = summarize_registered_test_failures(&output, &registered);
    assert!(summary.starts_with("cases::case_00, cases::case_01"));
    assert!(summary.contains("cases::case_31; 8 additional registered failures omitted"));
    assert!(!summary.contains("cases::case_32"));
    assert!(summary.len() < 1024);
}
