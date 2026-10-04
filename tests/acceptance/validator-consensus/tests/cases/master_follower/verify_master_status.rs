// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use eve_state::StateCommit;
use serde_json::{Value, json};

pub(super) fn verify_master_status(status: &Value, expected: &StateCommit) -> Result<()> {
    ensure!(
        status["role"] == "MASTER_SYNC_ONLY"
            && status["security_profile"] == "CLASSICAL_DEV"
            && status["verification_mode"] == "AUTHENTICATED_IMPORT"
            && status["authenticated_validator_finality"] == true,
        "MASTER_FOLLOWER_ROLE_OR_AUTHENTICATION"
    );
    for field in [
        "finalized_height",
        "applied_height",
        "durable_height",
        "authenticated_height",
    ] {
        ensure!(
            status[field] == expected.target.height,
            "MASTER_FOLLOWER_CONFIRMED_HEIGHT"
        );
    }
    ensure!(
        status["evm_root"] == json!(expected.target.evm_root.0)
            && status["system_root"] == json!(expected.target.system_root.0)
            && status["execution_hash"] == json!(expected.target.execution_hash.0)
            && status["content_digest"] == json!(expected.target.content_digest)
            && status["application_commitment"]
                == json!(expected.target.application.map(|value| value.0))
            && status["genesis_hash"] == json!(expected.target.identity.genesis.0),
        "MASTER_FOLLOWER_EXACT_ORACLE_BINDINGS"
    );
    ensure!(
        status["retained_proof_files"] == expected.target.height
            && status["retained_archive_bytes"]
                .as_u64()
                .is_some_and(|bytes| bytes > 0)
            && status["retained_canonical_commit_bytes"]
                .as_u64()
                .is_some_and(|bytes| bytes > 0),
        "MASTER_FOLLOWER_RETENTION_STATUS"
    );
    ensure!(
        status["ready"] == false
            && status["lag"].is_null()
            && status["peer_head"].is_null()
            && status["fenced"] == false
            && status["storage_outcome_unknown"] == false
            && status["rejected_staging_retained"] == false,
        "MASTER_FOLLOWER_FALSE_READINESS_OR_STORAGE_OUTCOME"
    );
    Ok(())
}
