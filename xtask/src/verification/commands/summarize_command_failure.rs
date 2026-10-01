// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Emit reviewed fixed diagnostic categories; raw command data stays local-only.
pub fn summarize_command_failure(stdout: &str, stderr: &str) -> String {
    let mut categories = Vec::new();
    for (needle, category) in [
        (
            "CometBFT binary digest mismatch",
            "ENGINE_BINARY_DIGEST_MISMATCH",
        ),
        (
            "CometBFT version command failed",
            "ENGINE_VERSION_COMMAND_FAILED",
        ),
        ("CometBFT init failed", "ENGINE_INITIALIZATION_FAILED"),
        ("B3_PHASE_FIXTURE", "B3_PHASE_FIXTURE"),
        ("B3_PHASE_INITIALIZATION", "B3_PHASE_INITIALIZATION"),
        ("B3_PHASE_PROXY", "B3_PHASE_PROXY"),
        ("B3_PHASE_NODE_SPAWN", "B3_PHASE_NODE_SPAWN"),
        ("B3_PHASE_ENGINE_DISCOVERY", "B3_PHASE_ENGINE_DISCOVERY"),
        (
            "owned native engine child not found",
            "ENGINE_DISCOVERY_DEADLINE",
        ),
        (
            "CometBFT exited before API smoke completed",
            "ENGINE_EXITED_DURING_LIFECYCLE",
        ),
        ("fixture CheckTx failed", "ABCI_CHECK_TX_FAILED"),
        (
            "restart replay duplicated",
            "ABCI_RESTART_DUPLICATED_TRANSACTION",
        ),
        ("timed out", "OPERATION_TIMED_OUT"),
        ("not Git-ignored", "ARTIFACT_IGNORE_GUARD_FAILED"),
        ("must remain beneath", "ARTIFACT_CONTAINMENT_FAILED"),
        ("checkpoint", "CHECKPOINT_CONTEXT_PRESENT"),
        (
            "API-test checkpoint did not persist",
            "APPLICATION_CHECKPOINT_BELOW_REQUIRED_HEIGHT",
        ),
        (
            "application checkpoint did not reach required height",
            "APPLICATION_CHECKPOINT_TIMED_OUT",
        ),
        ("Connection refused", "CONNECTION_REFUSED"),
        (
            "exited before height",
            "VALIDATOR_EXITED_BEFORE_REQUIRED_HEIGHT",
        ),
        (
            "native/application progress deadline",
            "CONSENSUS_PROGRESS_DEADLINE",
        ),
        (
            "actual native application/signer readiness deadline",
            "VALIDATOR_READINESS_DEADLINE",
        ),
        ("Permission denied", "PERMISSION_DENIED"),
        ("assertion", "ASSERTION_FAILED"),
    ] {
        if stdout.contains(needle) || stderr.contains(needle) {
            categories.push(category);
        }
    }
    if stdout
        .lines()
        .chain(stderr.lines())
        .any(|line| line.starts_with("error[E"))
    {
        categories.push("RUST_COMPILATION_FAILED");
    }
    categories.extend(
        super::summarize_runtime_failure_categories::summarize_runtime_failure_categories(
            stdout, stderr,
        ),
    );
    if categories.is_empty() {
        "UNCLASSIFIED_FAILURE; inspect ignored command evidence".into()
    } else {
        categories.join(", ")
    }
}
