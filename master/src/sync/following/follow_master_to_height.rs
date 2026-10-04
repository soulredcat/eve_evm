// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::import_master_wire;
use crate::sync::{
    MasterFollower, MasterSyncStatus, current_master_commit, master_sync_status,
    resources::reserve_master_bytes,
};
use anyhow::{Result, ensure};
use eve_state::{MAXIMUM_STATE_DELTA_CHUNK_BYTES, StateDeltaRequest};
use eve_sync_client::{
    NativeRpcConfig, downloaded_import_target, downloaded_import_wire,
    fetch_authenticated_import_wire, required_authenticated_import_download_reservation,
    validate_native_rpc_config,
};

/// Sequential finite offline catch-up. The shared client assembles untrusted actual
/// H/H+1 material; this role independently authenticates and archives every height.
pub fn follow_master_to_height(
    follower: &mut MasterFollower,
    rpc: NativeRpcConfig,
    maximum_chunk_bytes: u32,
    through_height: u64,
) -> Result<MasterSyncStatus> {
    ensure!(!follower.fenced, "MASTER_FENCED_REOPEN_REQUIRED");
    validate_native_rpc_config(rpc)?;
    ensure!(
        maximum_chunk_bytes > 0 && maximum_chunk_bytes as usize <= MAXIMUM_STATE_DELTA_CHUNK_BYTES,
        "MASTER_DOWNLOAD_CHUNK_PROFILE"
    );
    ensure!(
        through_height <= follower.config.maximum_proof_files && through_height < i64::MAX as u64,
        "MASTER_CATCHUP_HEIGHT_CAPACITY"
    );
    required_authenticated_import_download_reservation(rpc)?;
    while current_master_commit(follower).target.height < through_height {
        let parent = current_master_commit(follower).target.clone();
        let height = parent
            .height
            .checked_add(1)
            .ok_or_else(|| anyhow::anyhow!("MASTER_HEIGHT_ARITHMETIC"))?;
        let downloaded = fetch_authenticated_import_wire(
            rpc,
            StateDeltaRequest {
                parent,
                target_height: height,
                offset: 0,
                maximum_chunk_bytes,
            },
            &follower.config.storage.logical,
            |bytes| reserve_master_bytes(&follower.pool, bytes),
        )?;
        import_master_wire(
            follower,
            downloaded_import_wire(&downloaded),
            downloaded_import_target(&downloaded),
        )?;
    }
    Ok(master_sync_status(follower))
}
