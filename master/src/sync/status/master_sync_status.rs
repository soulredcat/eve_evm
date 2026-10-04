// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{MasterFollower, MasterSyncStatus, current_master_commit};
use eve_finality_verifier::imported_state_anchor;

/// This single-writer follower publishes only after proof and state syncs. Unknown peer
/// freshness remains explicit; a valid historical anchor does not establish readiness.
pub fn master_sync_status(follower: &MasterFollower) -> MasterSyncStatus {
    let commit = current_master_commit(follower);
    let authenticated = imported_state_anchor(&follower.current.state).is_some();
    MasterSyncStatus {
        role: "MASTER_SYNC_ONLY",
        security_profile: "CLASSICAL_DEV",
        verification_mode: "AUTHENTICATED_IMPORT",
        finalized_height: if authenticated {
            commit.target.height
        } else {
            0
        },
        applied_height: commit.target.height,
        durable_height: commit.target.height,
        authenticated_height: if authenticated {
            commit.target.height
        } else {
            0
        },
        authenticated_validator_finality: authenticated,
        target: commit.target.clone(),
        evm_root: commit.target.evm_root.0,
        system_root: commit.target.system_root.0,
        execution_hash: commit.target.execution_hash.0,
        content_digest: commit.target.content_digest,
        application_commitment: commit.target.application.map(|value| value.0),
        genesis_hash: commit.target.identity.genesis.0,
        retained_proof_files: follower.proof_count,
        retained_archive_bytes: follower.archive_bytes,
        retained_canonical_commit_bytes: follower.retained_commit_bytes,
        rejected_staging_retained: follower.rejected_staging,
        fenced: follower.fenced,
        storage_outcome_unknown: follower.fenced,
        lag: None,
        peer_head: None,
        ready: false,
    }
}
