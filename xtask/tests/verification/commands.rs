// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use xtask::verification::{
    commands::run_recorded_command::run_recorded_command, types::report_types::VerificationReport,
};

#[test]
fn missing_command_preserves_attempted_arguments_and_error_evidence() {
    let root = tempfile::tempdir().unwrap();
    let artifacts = tempfile::tempdir().unwrap();
    let mut report = VerificationReport::default();
    assert!(
        run_recorded_command(
            root.path(),
            artifacts.path(),
            &mut report,
            "eve_nonexistent_test_command_42",
            &["--required-case"]
        )
        .is_err()
    );
    assert_eq!(report.commands.len(), 1);
    let command = &report.commands[0];
    assert_eq!(command.arguments, ["--required-case"]);
    assert!(command.exit_code.is_none() && command.spawn_error.is_some());
    assert!(artifacts.path().join(&command.stdout).exists());
    assert!(
        !std::fs::read(artifacts.path().join(&command.stderr))
            .unwrap()
            .is_empty()
    );
    assert_eq!(command.stderr_sha256.len(), 64);
}

#[test]
fn compact_failure_categories_never_echo_raw_keys_paths_or_untrusted_text() {
    use xtask::verification::commands::summarize_command_failure::summarize_command_failure;
    let raw =
        "Error: CometBFT exited before API smoke completed; private-key=do-not-echo /personal/path";
    assert_eq!(
        summarize_command_failure(raw, ""),
        "ENGINE_EXITED_DURING_LIFECYCLE"
    );
    assert_eq!(
        summarize_command_failure("untrusted arbitrary bytes", "private-key=do-not-echo"),
        "UNCLASSIFIED_FAILURE; inspect ignored command evidence"
    );
    assert_eq!(
        summarize_command_failure("", "Connection refused; assertion failed"),
        "CONNECTION_REFUSED, ASSERTION_FAILED"
    );
    assert_eq!(
        summarize_command_failure("Error: API-test checkpoint did not persist", ""),
        "CHECKPOINT_CONTEXT_PRESENT, APPLICATION_CHECKPOINT_BELOW_REQUIRED_HEIGHT"
    );
}
