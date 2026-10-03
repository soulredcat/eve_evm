// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::BTreeSet;

/// Only exact error/context lines qualify; source excerpts and unknown text stay private.
pub(super) fn summarize_case_phase_failures(stdout: &str, stderr: &str) -> Vec<&'static str> {
    let mut phases = BTreeSet::new();
    for line in stdout.lines().chain(stderr.lines()) {
        let trimmed = line.trim();
        let mut context = trimmed.strip_prefix("Error: ").unwrap_or(trimmed);
        if let Some((index, value)) = context.split_once(": ")
            && !index.is_empty()
            && index.bytes().all(|byte| byte.is_ascii_digit())
        {
            context = value;
        }
        let code = context.split_once(": ").map_or(context, |(code, _)| code);
        let phase = match code {
            "B3_TC07_CREATE" => "B3_TC07_CREATE",
            "B3_TC07_BUILD_GUARDS" => "B3_TC07_BUILD_GUARDS",
            "B3_TC07_NORMAL_BINARY_REQUIRED" => "B3_TC07_NORMAL_BINARY_REQUIRED",
            "B3_TC07_FIXTURE_FLAG_REQUIRED" => "B3_TC07_FIXTURE_FLAG_REQUIRED",
            "B3_TC07_DEFAULT_REJECTION" => "B3_TC07_DEFAULT_REJECTION",
            "B3_TC07_DEFAULT_ACCEPTED" => "B3_TC07_DEFAULT_ACCEPTED",
            "B3_TC07_NORMAL_MISSING_MANIFEST" => "B3_TC07_NORMAL_MISSING_MANIFEST",
            "B3_TC07_NORMAL_MANIFEST_ACCEPTED" => "B3_TC07_NORMAL_MANIFEST_ACCEPTED",
            "B3_TC07_ACCEPTANCE_MISSING_MANIFEST" => "B3_TC07_ACCEPTANCE_MISSING_MANIFEST",
            "B3_TC07_ACCEPTANCE_MANIFEST_ACCEPTED" => "B3_TC07_ACCEPTANCE_MANIFEST_ACCEPTED",
            "B3_TC07_START" => "B3_TC07_START",
            "B3_TC07_SUBMIT" => "B3_TC07_SUBMIT",
            "B3_TC07_SUBMIT_ROTATION" => "B3_TC07_SUBMIT_ROTATION",
            "B3_TC07_SUBMIT_LEAVE" => "B3_TC07_SUBMIT_LEAVE",
            "B3_TC07_SUBMIT_JAIL" => "B3_TC07_SUBMIT_JAIL",
            "B3_TC07_ACTION_INVALID" => "B3_TC07_ACTION_INVALID",
            "B3_RPC_ENDPOINT" => "B3_RPC_ENDPOINT",
            "B3_RPC_REQUEST" => "B3_RPC_REQUEST",
            "B3_RPC_CONNECT" => "B3_RPC_CONNECT",
            "B3_RPC_WRITE" => "B3_RPC_WRITE",
            "B3_RPC_READ" => "B3_RPC_READ",
            "B3_RPC_JSON" => "B3_RPC_JSON",
            "B3_RPC_ERROR" => "B3_RPC_ERROR",
            "B3_RPC_RESULT" => "B3_RPC_RESULT",
            "B3_RPC_IO_TIMEOUT" => "B3_RPC_IO_TIMEOUT",
            "B3_RPC_IO_DISCONNECTED" => "B3_RPC_IO_DISCONNECTED",
            "B3_RPC_IO_REFUSED" => "B3_RPC_IO_REFUSED",
            "B3_RPC_IO_PERMISSION" => "B3_RPC_IO_PERMISSION",
            "B3_RPC_IO_OTHER" => "B3_RPC_IO_OTHER",
            "B3_SUBMIT_CHECK_TX_MISSING" => "B3_SUBMIT_CHECK_TX_MISSING",
            "B3_SUBMIT_CHECK_TX_CODE" => "B3_SUBMIT_CHECK_TX_CODE",
            "B3_SUBMIT_CHECK_TX_REJECTED" => "B3_SUBMIT_CHECK_TX_REJECTED",
            "B3_SUBMIT_TX_RESULT_MISSING" => "B3_SUBMIT_TX_RESULT_MISSING",
            "B3_SUBMIT_TX_RESULT_CODE" => "B3_SUBMIT_TX_RESULT_CODE",
            "B3_SUBMIT_TX_RESULT_REJECTED" => "B3_SUBMIT_TX_RESULT_REJECTED",
            "B3_SUBMIT_HASH_MISSING" => "B3_SUBMIT_HASH_MISSING",
            "B3_SUBMIT_HASH_FORMAT" => "B3_SUBMIT_HASH_FORMAT",
            "B3_SUBMIT_HASH_MISMATCH" => "B3_SUBMIT_HASH_MISMATCH",
            "B3_SUBMIT_HEIGHT_MISSING" => "B3_SUBMIT_HEIGHT_MISSING",
            "B3_SUBMIT_HEIGHT_INVALID" => "B3_SUBMIT_HEIGHT_INVALID",
            "B3_TC07_CERTIFICATE_HISTORY" => "B3_TC07_CERTIFICATE_HISTORY",
            "B3_TC07_REPLAY" => "B3_TC07_REPLAY",
            "B3_TC07_EXECUTION_MISSING" => "B3_TC07_EXECUTION_MISSING",
            "B3_TC07_TRANSACTION_MISSING" => "B3_TC07_TRANSACTION_MISSING",
            "B3_TC07_RECEIPT" => "B3_TC07_RECEIPT",
            "B3_TC07_ROSTER" => "B3_TC07_ROSTER",
            "B3_TC07_ROSTER_HASH" => "B3_TC07_ROSTER_HASH",
            "B3_TC07_PROGRESS" => "B3_TC07_PROGRESS",
            "B3_TC07_FINAL_HISTORY" => "B3_TC07_FINAL_HISTORY",
            "B3_TC07_HISTORY_MISSING" => "B3_TC07_HISTORY_MISSING",
            "B3_TC07_NEXT_SET_HASH" => "B3_TC07_NEXT_SET_HASH",
            "B3_TC07_ACTIVE_SET_HASH" => "B3_TC07_ACTIVE_SET_HASH",
            "B3_TC07_PREVIOUS_COMMIT_MISSING" => "B3_TC07_PREVIOUS_COMMIT_MISSING",
            "B3_TC07_PREVIOUS_COMMIT" => "B3_TC07_PREVIOUS_COMMIT",
            "B3_TC07_FINAL_REPLAY" => "B3_TC07_FINAL_REPLAY",
            "B3_TC07_REPLAY_EMPTY" => "B3_TC07_REPLAY_EMPTY",
            "B3_TC07_AUTHORITY_MISSING" => "B3_TC07_AUTHORITY_MISSING",
            "B3_TC07_NONCE" => "B3_TC07_NONCE",
            "B3_TC07_STOP" => "B3_TC07_STOP",
            "B3_TC07_CALLBACKS" => "B3_TC07_CALLBACKS",
            "B3_TC07_CALLBACK_MISSING" => "B3_TC07_CALLBACK_MISSING",
            "B3_TC07_DECIDED_COMMIT_MISSING" => "B3_TC07_DECIDED_COMMIT_MISSING",
            "B3_TC07_CALLBACK_ROSTER" => "B3_TC07_CALLBACK_ROSTER",
            "B3_TC07_VOTE_VALIDATOR_MISSING" => "B3_TC07_VOTE_VALIDATOR_MISSING",
            "B3_TC07_CALLBACK_VOTE" => "B3_TC07_CALLBACK_VOTE",
            "B3_TC07_STORES" => "B3_TC07_STORES",
            _ => continue,
        };
        phases.insert(phase);
    }
    phases.into_iter().collect()
}
