// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Store only reviewed fixed categories. Unknown displays, paths, packets and key material stay redacted.
pub(in crate::consensus::runtime) fn classify_node_failure(error: &str) -> &'static str {
    match error {
        "native_signer_actor" => "SIGNER_ACTOR",
        "native_application_actor" => "APPLICATION_ACTOR",
        "signer_frame_read" => "SIGNER_READ",
        "signer_frame_write" => "SIGNER_WRITE",
        "signer_dispatch" => "SIGNER_DISPATCH",
        "application_frame_read" => "APPLICATION_READ",
        "application_frame_write" => "APPLICATION_WRITE",
        "application_dispatch" => "APPLICATION_DISPATCH",
        "signer height/round/step regression" => "HRS_REGRESSION",
        "conflicting vote at identical height/round/step" => "VOTE_CONFLICT",
        "conflicting proposal at identical height/round/step" => "PROPOSAL_CONFLICT",
        "signer fenced; reopen and reconcile required" => "SIGNER_FENCED",
        "non-nil vote requires canonical execution/data approval" => "APPROVAL_REQUIRED",
        "required proposal execution/data unavailable" => "APPROVAL_DATA_UNAVAILABLE",
        "approval cache unavailable" => "APPROVAL_CACHE_UNAVAILABLE",
        "proposal approval cache failed" => "APPROVAL_CACHE_FAILED",
        "retained proposal execution failed" => "APPROVAL_REEXECUTION_FAILED",
        "prepared proposal vote signing failed" => "APPROVAL_SIGNING_FAILED",
        "execution approval parent is stale" => "APPROVAL_STALE_PARENT",
        "new signed vote retention pin failed" => "VOTE_RETENTION_FAILED",
        "native application connection capacity exceeded" => "APPLICATION_CONNECTION_CAPACITY",
        "actual native application/signer readiness deadline exceeded" => "READINESS_DEADLINE",
        "native signer reconnect deadline exceeded" => "SIGNER_RECONNECT_DEADLINE",
        "application state lacks matching consensus replay history" => "REPLAY_HEAD_MISMATCH",
        "replayed consensus block differs from exact decided input" => "REPLAY_INPUT_MISMATCH",
        "different finalization before pending commit" => "PENDING_INPUT_MISMATCH",
        "application request height differs from actual parent" => "APPLICATION_PARENT_HEIGHT",
        "signer history is fenced or ahead of canonical application recovery" => {
            "SIGNER_AHEAD_OF_RECOVERY"
        }
        "signer chain mismatch" => "SIGNER_CHAIN_MISMATCH",
        "current signer parent unavailable" => "SIGNER_PARENT_UNAVAILABLE",
        "signer request height differs from current execution parent" => "SIGNER_PARENT_HEIGHT",
        "new signature not at current execution height" => "SIGNER_PARENT_HEIGHT",
        "decided block canonical execution failed" => "DECIDED_EXECUTION_FAILED",
        "application unavailable for proposal execution" => "APPLICATION_UNAVAILABLE",
        "owned engine exited; private diagnostics preserved" => "ENGINE_EXITED",
        "native channel worker terminated unexpectedly" => "WORKER_UNWIND",
        _ => "REDACTED_UNCLASSIFIED",
    }
}
