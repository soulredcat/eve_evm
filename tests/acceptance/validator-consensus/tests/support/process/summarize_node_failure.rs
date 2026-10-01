// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::read_owned_failure_metadata::read_owned_failure_metadata;
use std::{collections::BTreeSet, path::Path};

/// Failure metadata is diagnostic only. Never return raw messages, keys, paths or arbitrary fields.
pub(crate) fn summarize_node_failure(data: &Path, process_id: u32) -> String {
    let Some(record) = read_owned_failure_metadata(data, process_id) else {
        return "NODE_FAILURE_RECORD_UNAVAILABLE".into();
    };
    let Some(categories) = record["categories"]
        .as_array()
        .filter(|array| array.len() <= 8)
    else {
        return "NODE_FAILURE_RECORD_INVALID".into();
    };
    let mut codes = BTreeSet::new();
    for category in categories {
        let known = category.as_str().filter(|value| {
            matches!(
                *value,
                "SIGNER_ACTOR"
                    | "APPLICATION_ACTOR"
                    | "SIGNER_READ"
                    | "SIGNER_WRITE"
                    | "SIGNER_DISPATCH"
                    | "APPLICATION_READ"
                    | "APPLICATION_WRITE"
                    | "APPLICATION_DISPATCH"
                    | "HRS_REGRESSION"
                    | "VOTE_CONFLICT"
                    | "PROPOSAL_CONFLICT"
                    | "SIGNER_FENCED"
                    | "APPROVAL_REQUIRED"
                    | "APPROVAL_DATA_UNAVAILABLE"
                    | "APPROVAL_CACHE_UNAVAILABLE"
                    | "APPROVAL_STALE_PARENT"
                    | "APPROVAL_CACHE_FAILED"
                    | "APPROVAL_REEXECUTION_FAILED"
                    | "APPROVAL_SIGNING_FAILED"
                    | "VOTE_RETENTION_FAILED"
                    | "APPLICATION_CONNECTION_CAPACITY"
                    | "READINESS_DEADLINE"
                    | "SIGNER_RECONNECT_DEADLINE"
                    | "REPLAY_HEAD_MISMATCH"
                    | "REPLAY_INPUT_MISMATCH"
                    | "PENDING_INPUT_MISMATCH"
                    | "APPLICATION_PARENT_HEIGHT"
                    | "SIGNER_AHEAD_OF_RECOVERY"
                    | "SIGNER_CHAIN_MISMATCH"
                    | "SIGNER_PARENT_UNAVAILABLE"
                    | "SIGNER_PARENT_HEIGHT"
                    | "DECIDED_EXECUTION_FAILED"
                    | "APPLICATION_UNAVAILABLE"
                    | "ENGINE_EXITED"
                    | "WORKER_UNWIND"
                    | "REDACTED_UNCLASSIFIED"
            )
        });
        codes.insert(known.unwrap_or("REDACTED_UNCLASSIFIED"));
    }
    if let Some(kind) = record["io_kind"].as_str() {
        codes.insert(match kind {
            "WouldBlock" => "IO_WOULD_BLOCK",
            "TimedOut" => "IO_TIMED_OUT",
            "PermissionDenied" => "IO_PERMISSION_DENIED",
            "ConnectionReset" => "IO_CONNECTION_RESET",
            "BrokenPipe" => "IO_BROKEN_PIPE",
            "NotFound" => "IO_NOT_FOUND",
            _ => "REDACTED_IO_KIND",
        });
    }
    if codes.is_empty() {
        "NODE_FAILURE_RECORD_INVALID".into()
    } else {
        codes.into_iter().collect::<Vec<_>>().join(",")
    }
}
