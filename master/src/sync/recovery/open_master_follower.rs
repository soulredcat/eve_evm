// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::reconcile_master_archive::reconcile_master_archive;
use crate::{
    development::config::resolve_development_directory::resolve_development_directory,
    sync::{
        MasterFollower, MasterSyncConfig,
        archive::{open_proof_directory, scan_proof_archive, sync_master_namespace},
        config::validate_master_sync_config,
        resources::{reserve_master_bytes, storage_working_bytes},
        types::RetainedImport,
    },
};
use anyhow::{Result, ensure};
use eve_finality_verifier::{imported_state_commit, initialize_authenticated_import};
use eve_state::{
    BOUNDED_STATE_CODEC_SCRATCH_BYTES, DevelopmentGenesis, encode_state_commit,
    estimate_genesis_initialization_reservation,
};
use eve_storage::state::{open_state_repository, state_reader};
use std::sync::Arc;
use tokio::sync::Semaphore;

pub fn open_master_follower(
    config: MasterSyncConfig,
    genesis: &DevelopmentGenesis,
) -> Result<MasterFollower> {
    validate_master_sync_config(&config, genesis)?;
    let pool = Arc::new(Semaphore::new(config.working_bytes / 1_024));
    let archive_lease = reserve_master_bytes(&pool, storage_working_bytes(&config.storage)?)?;
    let scratch = reserve_master_bytes(&pool, BOUNDED_STATE_CODEC_SCRATCH_BYTES)?;
    let genesis_bytes =
        estimate_genesis_initialization_reservation(genesis, &config.storage.logical)
            .map_err(|error| anyhow::anyhow!("MASTER_GENESIS_RESOURCE: {error:?}"))?;
    let lease = reserve_master_bytes(
        &pool,
        genesis_bytes
            .checked_mul(3)
            .ok_or_else(|| anyhow::anyhow!("MASTER_GENESIS_RESOURCE_ARITHMETIC"))?,
    )?;
    let state = Arc::new(
        initialize_authenticated_import(genesis, &config.storage.logical)
            .map_err(|error| anyhow::anyhow!("MASTER_LOCAL_GENESIS: {error:?}"))?,
    );
    let retained_commit_bytes = u64::try_from(
        encode_state_commit(imported_state_commit(&state), &config.storage.logical)
            .map_err(|error| anyhow::anyhow!("MASTER_GENESIS_COMMIT: {error:?}"))?
            .len(),
    )?;
    ensure!(
        retained_commit_bytes <= config.maximum_retained_commit_bytes,
        "MASTER_COMMIT_ARCHIVE_BYTE_CAPACITY"
    );
    drop(scratch);
    let path = resolve_development_directory(&config.root, &config.data)?;
    std::fs::create_dir_all(&path)?;
    let checked = resolve_development_directory(&config.root, &path)?;
    ensure!(checked == path, "MASTER_NAMESPACE_CHANGED");
    let proofs = path.join("proofs");
    if !proofs.exists() {
        std::fs::create_dir(&proofs)?;
    }
    let directory = open_proof_directory(&proofs)?;
    // Sync directory creation before proof/state admission.
    sync_master_namespace(&path, &config.root)?;
    let inventory = scan_proof_archive(&directory, &config)?;
    let database = path.join("state");
    if let Ok(metadata) = std::fs::symlink_metadata(&database) {
        ensure!(
            metadata.is_dir() && !metadata.file_type().is_symlink(),
            "MASTER_STATE_NAMESPACE"
        );
    }
    let repository =
        open_state_repository(&database, imported_state_commit(&state), config.storage)?;
    sync_master_namespace(&path, &config.root)?;
    let reader = state_reader(&repository);
    let mut follower = MasterFollower {
        current: RetainedImport {
            state,
            _lease: lease,
        },
        repository,
        reader,
        directory,
        config,
        proof_count: inventory.completed,
        archive_bytes: inventory.bytes,
        retained_commit_bytes,
        rejected_staging: inventory.rejected,
        fenced: false,
        pool,
        #[cfg(test)]
        fault: None,
        _archive_lease: archive_lease,
    };
    reconcile_master_archive(&mut follower, inventory)?;
    Ok(follower)
}
