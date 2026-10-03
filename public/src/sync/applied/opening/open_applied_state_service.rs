// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    validate_applied_configuration::validate_applied_configuration,
    validate_local_genesis_resources::validate_local_genesis_resources,
};
use crate::{
    persistence::{handoff::create_handoff_pool, worker::start_record_worker},
    sync::applied::{
        AppliedConfig, AppliedError, AppliedOwner, AppliedPublication, AppliedReader,
        publication::build_applied_markers,
        recovery::recover_applied_prefix,
        resources::{
            create_estimated_working_pool, estimate_pending_metadata, estimate_replay_charge,
            estimated_clone_ceiling, estimated_repository_read_charge, reserve_estimated_working,
            split_estimated_working,
        },
        types::ChargedRecoveryState,
    },
};
use eve_finality_verifier::{initialize_development_recovery, recovery_state_commit};
use eve_state::DevelopmentGenesis;
use eve_storage::records::{opaque_record_cursor, open_opaque_record_repository};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

/// Open the declared namespace, replay its complete verified prefix, then start its sole writer.
/// This local empty-block capability does not establish independent fresh-head readiness.
pub fn open_applied_state_service(
    config: AppliedConfig,
    genesis: &DevelopmentGenesis,
) -> Result<(AppliedOwner, AppliedReader), AppliedError> {
    validate_applied_configuration(&config)?;
    validate_local_genesis_resources(genesis, &config.state_budget)?;
    let working = create_estimated_working_pool(config.public_budget.maximum_working_state_bytes)?;
    let charge = estimate_replay_charge(
        &config.state_budget,
        config.maximum_recovery_payload_bytes,
        estimated_clone_ceiling(&config.state_budget)?,
    )?;
    let lease = reserve_estimated_working(&working, charge.total)?;
    let recovery = initialize_development_recovery(genesis, &config.state_budget)
        .map_err(AppliedError::Recovery)?;
    if recovery_state_commit(&recovery).target.identity.genesis.0.0 != config.identity.genesis_hash
    {
        return Err(AppliedError::InvalidConfiguration);
    }
    let (retained, transient) = split_estimated_working(lease, charge.retained)?;
    let generation = Arc::new(ChargedRecoveryState {
        recovery: Arc::new(recovery),
        _lease: retained,
    });
    drop(transient);
    let pool = create_handoff_pool(config.public_budget).map_err(AppliedError::Handoff)?;
    let read_lease = reserve_estimated_working(
        &working,
        estimated_repository_read_charge(&config.repository_budget)?,
    )?;
    let repository =
        open_opaque_record_repository(&config.path, config.identity, config.repository_budget)
            .map_err(|_| AppliedError::StorageUnavailable)?;
    let cursor = opaque_record_cursor(&repository).map_err(|_| AppliedError::StorageUnavailable)?;
    drop(read_lease);
    let generation = recover_applied_prefix(&config, &repository, generation, &working)?;
    let height = recovery_state_commit(&generation.recovery).target.height;
    let markers = build_applied_markers(&generation.recovery, height)?;
    let publication = Arc::new(RwLock::new(Arc::new(AppliedPublication {
        generation,
        markers,
        durable_cursor: cursor,
        admitted_cursor: cursor,
        storage_failed: false,
    })));
    let reader = AppliedReader {
        publication,
        working,
    };
    let capacity = usize::try_from(config.public_budget.queue_batches)
        .map_err(|_| AppliedError::InvalidConfiguration)?;
    let metadata_lease =
        reserve_estimated_working(&reader.working, estimate_pending_metadata(capacity)?)?;
    let mut pending = VecDeque::new();
    pending
        .try_reserve_exact(capacity)
        .map_err(|_| AppliedError::AllocationFailed)?;
    if pending.capacity() != capacity {
        return Err(AppliedError::AllocationFailed);
    }
    let worker = start_record_worker(repository, Arc::clone(&pool), config.worker_scratch_limit)
        .map_err(AppliedError::Worker)?;
    let owner = AppliedOwner {
        config,
        worker: Some(worker),
        pool,
        reader: reader.clone(),
        pending,
        admitted_cursor: cursor,
        durable_cursor: cursor,
        database_sequence: 0,
        storage_failed: false,
        metadata_lease,
    };
    Ok((owner, reader))
}
