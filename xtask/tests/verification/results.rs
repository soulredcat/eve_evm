// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use xtask::verification::discovery::parse_test_results::parse_test_results;

#[test]
fn fixed_case_phases_require_error_context_lines_and_redact_unknown_data() {
    use xtask::verification::commands::summarize_command_failure::summarize_command_failure;
    let summary = summarize_command_failure(
        "",
        "Error: B3_TC07_BUILD_GUARDS\n\nCaused by:\n    0: B3_TC07_NORMAL_MISSING_MANIFEST\n    1: private-sensitive-value\n",
    );
    assert_eq!(
        summary,
        "B3_TC07_BUILD_GUARDS, B3_TC07_NORMAL_MISSING_MANIFEST"
    );
    assert!(!summary.contains("private-sensitive-value"));
    assert_eq!(
        summarize_command_failure(
            "",
            "Error: B3_TC07_SUBMIT_JAIL\nCaused by:\n    0: B3_RPC_READ\n    1: B3_RPC_IO_TIMEOUT\n    2: private-sensitive-value"
        ),
        "B3_RPC_IO_TIMEOUT, B3_RPC_READ, B3_TC07_SUBMIT_JAIL"
    );
    assert_eq!(
        summarize_command_failure(
            "",
            "Error: B3_SUBMIT_CHECK_TX_CODE: private-sensitive-value"
        ),
        "B3_SUBMIT_CHECK_TX_CODE"
    );
    assert_eq!(
        summarize_command_failure(
            "",
            "Error: B3_PROGRESS_DEADLINE: private-sensitive-value\nCaused by:\n    0: B3_SUBMIT_BLOCK_RESULTS_COUNT: private-sensitive-value"
        ),
        "B3_PROGRESS_DEADLINE, B3_SUBMIT_BLOCK_RESULTS_COUNT"
    );
    assert_eq!(
        summarize_command_failure(
            "",
            "Error: B3_TC07_CALLBACK_MISSING: private-sensitive-value"
        ),
        "B3_TC07_CALLBACK_MISSING"
    );
    let source_diff = "+ .context(\"B3_TC07_SUBMIT\")?;\n- \"B3_TC07_CALLBACK_MISSING\"";
    let summary = summarize_command_failure(source_diff, "unclassified private-sensitive-value");
    assert_eq!(
        summary,
        "UNCLASSIFIED_FAILURE; inspect ignored command evidence"
    );
    assert!(!summary.contains("private-sensitive-value"));
}

#[test]
fn results_require_exact_nonzero_execution_and_zero_skipped_or_failed_cases() {
    let valid = "test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0s\ntest result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0s\n";
    assert_eq!(parse_test_results(valid, 2).unwrap(), 2);
    assert!(parse_test_results(valid, 3).is_err());
    assert!(parse_test_results("", 0).is_err());
    for replacement in ["1 failed", "1 ignored", "1 measured", "1 filtered out"] {
        let failed = valid.replace(&replacement.replacen('1', "0", 1), replacement);
        assert!(parse_test_results(&failed, 2).is_err(), "{replacement}");
    }
    assert!(parse_test_results(&valid.replace("ok.", "FAILED."), 2).is_err());
    assert!(parse_test_results("test result: ok. no counts\n", 2).is_err());
}
