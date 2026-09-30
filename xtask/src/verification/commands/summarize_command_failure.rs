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
        ("Connection refused", "CONNECTION_REFUSED"),
        ("Permission denied", "PERMISSION_DENIED"),
        ("assertion", "ASSERTION_FAILED"),
        ("error[E", "RUST_COMPILATION_FAILED"),
    ] {
        if stdout.contains(needle) || stderr.contains(needle) {
            categories.push(category);
        }
    }
    if categories.is_empty() {
        "UNCLASSIFIED_FAILURE; inspect ignored command evidence".into()
    } else {
        categories.join(", ")
    }
}
