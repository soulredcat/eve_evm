// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::BTreeSet;

/// Accept only a bounded explicit harness marker, never tokens found in source diffs or payloads.
pub(super) fn summarize_runtime_failure_categories(
    stdout: &str,
    stderr: &str,
) -> Vec<&'static str> {
    let mut codes = BTreeSet::new();
    for line in stdout.lines().chain(stderr.lines()) {
        let Some((_, tail)) = line.split_once("failure categories ") else {
            continue;
        };
        let Some((text, _)) = tail.split_once("; artifacts ") else {
            continue;
        };
        if text.len() > 512 {
            continue;
        }
        for value in text.split(',').take(9) {
            let code = match value.trim() {
                "HRS_REGRESSION" => "HRS_REGRESSION",
                "SIGNER_PARENT_HEIGHT" => "SIGNER_PARENT_HEIGHT",
                "SIGNER_PARENT_UNAVAILABLE" => "SIGNER_PARENT_UNAVAILABLE",
                "SIGNER_AHEAD_OF_RECOVERY" => "SIGNER_AHEAD_OF_RECOVERY",
                "SIGNER_ACTOR" => "SIGNER_ACTOR",
                "APPLICATION_ACTOR" => "APPLICATION_ACTOR",
                "SIGNER_READ" => "SIGNER_READ",
                "SIGNER_WRITE" => "SIGNER_WRITE",
                "SIGNER_DISPATCH" => "SIGNER_DISPATCH",
                "APPLICATION_READ" => "APPLICATION_READ",
                "APPLICATION_WRITE" => "APPLICATION_WRITE",
                "APPLICATION_DISPATCH" => "APPLICATION_DISPATCH",
                "APPROVAL_REEXECUTION_FAILED" => "APPROVAL_REEXECUTION_FAILED",
                "APPROVAL_CACHE_FAILED" => "APPROVAL_CACHE_FAILED",
                "APPROVAL_SIGNING_FAILED" => "APPROVAL_SIGNING_FAILED",
                "APPROVAL_REQUIRED" => "APPROVAL_REQUIRED",
                "APPROVAL_STALE_PARENT" => "APPROVAL_STALE_PARENT",
                "VOTE_RETENTION_FAILED" => "VOTE_RETENTION_FAILED",
                "SIGNER_RECONNECT_DEADLINE" => "SIGNER_RECONNECT_DEADLINE",
                "READINESS_DEADLINE" => "READINESS_DEADLINE",
                "REPLAY_HEAD_MISMATCH" => "REPLAY_HEAD_MISMATCH",
                "REPLAY_INPUT_MISMATCH" => "REPLAY_INPUT_MISMATCH",
                "SIGNER_FENCED" => "SIGNER_FENCED",
                "ENGINE_EXITED" => "ENGINE_EXITED",
                "IO_WOULD_BLOCK" => "IO_WOULD_BLOCK",
                "IO_TIMED_OUT" => "IO_TIMED_OUT",
                "IO_CONNECTION_RESET" => "IO_CONNECTION_RESET",
                "IO_BROKEN_PIPE" => "IO_BROKEN_PIPE",
                "IO_PERMISSION_DENIED" => "IO_PERMISSION_DENIED",
                "IO_NOT_FOUND" => "IO_NOT_FOUND",
                "NODE_FAILURE_RECORD_UNAVAILABLE" => "NODE_FAILURE_RECORD_UNAVAILABLE",
                "NODE_FAILURE_RECORD_INVALID" => "NODE_FAILURE_RECORD_INVALID",
                _ => "REDACTED_RUNTIME_CATEGORY",
            };
            codes.insert(code);
        }
    }
    codes.into_iter().collect()
}
