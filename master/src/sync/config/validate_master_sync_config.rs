// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{MasterSyncConfig, resources::storage_working_bytes};
use anyhow::{Result, ensure};
use eve_finality_verifier::MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES;
use eve_state::{BOUNDED_STATE_CODEC_SCRATCH_BYTES, DevelopmentGenesis, SecurityProfile};
use tokio::sync::Semaphore;

pub(in crate::sync) fn validate_master_sync_config(
    config: &MasterSyncConfig,
    genesis: &DevelopmentGenesis,
) -> Result<()> {
    ensure!(
        cfg!(target_os = "linux"),
        "MASTER_ARCHIVE_PLATFORM_REQUIRES_LINUX_DIRECTORY_SYNC"
    );
    ensure!(
        config.mode == "MASTER_SYNC_ONLY" && config.acknowledge_unsafe_development,
        "MASTER_SYNC_ONLY requires explicit unsafe classical-development acknowledgement"
    );
    eve_storage::state::validate_state_storage_budget(&config.storage)?;
    ensure!(
        genesis.profile == SecurityProfile::ClassicalDev,
        "MASTER_SYNC_ONLY currently requires CLASSICAL_DEV"
    );
    ensure!(
        config.working_bytes > 0
            && config.working_bytes.is_multiple_of(1_024)
            && config.working_bytes / 1_024 <= Semaphore::MAX_PERMITS
            && u32::try_from(config.working_bytes / 1_024).is_ok(),
        "MASTER_WORKING_PROFILE"
    );
    ensure!(
        config.maximum_proof_bytes > 0
            && config.maximum_proof_bytes <= MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES
            && config.maximum_proof_files > 0
            && config.maximum_proof_files <= 1_000_000,
        "MASTER_PROOF_PROFILE"
    );
    let headroom = u64::try_from(config.maximum_proof_bytes)?
        .checked_mul(2)
        .ok_or_else(|| anyhow::anyhow!("MASTER_ARCHIVE_ARITHMETIC"))?;
    ensure!(
        config.maximum_archive_bytes >= headroom,
        "MASTER_ARCHIVE_STAGING_HEADROOM"
    );
    ensure!(
        config.maximum_retained_commit_bytes
            >= u64::try_from(config.storage.maximum_commit_bytes)?
                .checked_mul(2)
                .ok_or_else(|| anyhow::anyhow!("MASTER_ARCHIVE_ARITHMETIC"))?,
        "MASTER_COMMIT_ARCHIVE_HEADROOM"
    );
    let minimum = storage_working_bytes(&config.storage)?
        .checked_add(BOUNDED_STATE_CODEC_SCRATCH_BYTES)
        .and_then(|value| value.checked_add(config.maximum_proof_bytes))
        .ok_or_else(|| anyhow::anyhow!("MASTER_RESOURCE_ARITHMETIC"))?;
    ensure!(
        config.working_bytes >= minimum,
        "MASTER_WORKING_PROFILE_CAPACITY"
    );
    ensure!(
        config.storage.logical.maximum_commit_bytes <= config.storage.maximum_commit_bytes
            && config.storage.maximum_commit_bytes <= config.storage.database.max_batch_bytes
            && config.storage.maximum_snapshots > 0,
        "MASTER_STORAGE_PROFILE"
    );
    Ok(())
}
