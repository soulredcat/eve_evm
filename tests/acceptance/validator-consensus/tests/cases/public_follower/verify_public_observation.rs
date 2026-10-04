// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::PublicObservation;
use alloy_primitives::Address;
use anyhow::{Result, ensure};
use eve_state::StateCommit;
use serde_json::json;

pub(super) fn verify_public_observation(
    observation: &PublicObservation,
    expected: &StateCommit,
    sender: Address,
) -> Result<()> {
    ensure!(
        observation.roots["height"] == format!("0x{:x}", expected.target.height),
        "PUBLIC_FOLLOWER_ROOT_HEIGHT_MISMATCH"
    );
    ensure!(
        observation.roots["evmRoot"] == json!(expected.target.evm_root.0)
            && observation.roots["systemRoot"] == json!(expected.target.system_root.0)
            && observation.roots["executionHash"] == json!(expected.target.execution_hash.0)
            && observation.roots["contentDigest"] == json!(expected.target.content_digest),
        "PUBLIC_FOLLOWER_COMPLETE_ROOT_MISMATCH"
    );
    ensure!(
        observation.roots["applicationCommitment"]
            == json!(expected.target.application.map(|value| value.0)),
        "PUBLIC_FOLLOWER_APPLICATION_COMMITMENT_MISMATCH"
    );
    ensure!(
        observation.roots["authenticatedFinality"] == true
            && observation.roots["verificationMode"] == "AUTHENTICATED_IMPORT",
        "PUBLIC_FOLLOWER_VERIFICATION_MODE"
    );
    ensure!(
        observation.block["hash"] == json!(expected.target.execution_hash.0)
            && observation.block["stateRoot"] == json!(expected.target.evm_root.0),
        "PUBLIC_FOLLOWER_BLOCK_ROOT_MISMATCH"
    );
    let account = expected
        .state
        .accounts
        .get(&sender)
        .ok_or_else(|| anyhow::anyhow!("PUBLIC_FOLLOWER_ORACLE_SENDER"))?;
    ensure!(
        observation.sender_balance == format!("0x{:x}", account.balance)
            && observation.sender_nonce == format!("0x{:x}", account.nonce),
        "PUBLIC_FOLLOWER_ACCOUNT_MISMATCH"
    );
    let applied = observation.status["applied_height"]
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("PUBLIC_FOLLOWER_STATUS_HEIGHT"))?;
    let durable = observation.status["durable_height"]
        .as_u64()
        .ok_or_else(|| anyhow::anyhow!("PUBLIC_FOLLOWER_STATUS_DURABLE"))?;
    ensure!(
        applied == expected.target.height
            && durable <= applied
            && observation.status["authenticated_height"] == applied
            && observation.status["finalized_height"] == applied + 1,
        "PUBLIC_FOLLOWER_WATERMARKS"
    );
    ensure!(
        observation.status["ready"] == false
            && observation.status["readiness_reason"] == "head freshness unknown"
            && observation.status["peer_count"].is_null()
            && observation.status["lag"].is_null()
            && observation.status["storage_failed"] == false,
        "PUBLIC_FOLLOWER_FALSE_FRESHNESS_OR_DURABILITY"
    );
    Ok(())
}
