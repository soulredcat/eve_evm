// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    derive_effective_storage_identity::derive_effective_storage_identity,
    initialize_charged_generation::initialize_charged_generation,
    validate_applied_configuration::validate_applied_configuration,
    validate_local_genesis_resources::validate_local_genesis_resources,
};
use crate::{
    persistence::{handoff::create_handoff_pool, worker::start_record_worker},
    sync::applied::{
        AppliedConfig, AppliedError, AppliedMode, AppliedOwner, AppliedPublication, AppliedReader,
        publication::build_applied_markers,
        recovery::recover_applied_prefix,
        resources::{
            create_estimated_working_pool, estimate_pending_metadata,
            estimated_repository_read_charge, reserve_estimated_working,
        },
        state::applied_state_commit,
    },
};
use eve_state::DevelopmentGenesis;
use eve_storage::records::{opaque_record_cursor, open_opaque_record_repository};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

/// Explicit local mode; authenticate the full declared namespace before starting its sole writer.
pub fn open_applied_state_service_with_mode(
    config: AppliedConfig,
    genesis: &DevelopmentGenesis,
    mode: AppliedMode,
) -> Result<(AppliedOwner, AppliedReader), AppliedError> {
    validate_applied_configuration(&config, mode)?;
    validate_local_genesis_resources(genesis, &config.state_budget)?;
    let working = create_estimated_working_pool(config.public_budget.maximum_working_state_bytes)?;
    let storage = super::super::resources::storage_admission::create_storage_admission_pool(
        config.public_budget,
        &working,
    )
    .map_err(super::super::resources::storage_admission::map_storage_admission_error)?;
    let generation = initialize_charged_generation(&config, genesis, mode, &working)?;
    if applied_state_commit(&generation.state)
        .target
        .identity
        .genesis
        .0
        .0
        != config.identity.genesis_hash
    {
        return Err(AppliedError::InvalidConfiguration);
    }
    let effective_storage_identity = derive_effective_storage_identity(config.identity, mode);
    let pool = create_handoff_pool(config.public_budget).map_err(AppliedError::Handoff)?;
    let read_lease = reserve_estimated_working(
        &working,
        estimated_repository_read_charge(&config.repository_budget)?,
    )?;
    let _startup_read = super::super::resources::storage_admission::reserve_storage_read(&storage)
        .map_err(super::super::resources::storage_admission::map_storage_admission_error)?;
    let repository = open_opaque_record_repository(
        &config.path,
        effective_storage_identity,
        config.repository_budget,
    )
    .map_err(|_| AppliedError::StorageUnavailable)?;
    drop(_startup_read);
    let cursor = opaque_record_cursor(&repository).map_err(|_| AppliedError::StorageUnavailable)?;
    drop(read_lease);
    let generation = {
        let _read = super::super::resources::storage_admission::reserve_storage_read(&storage)
            .map_err(super::super::resources::storage_admission::map_storage_admission_error)?;
        recover_applied_prefix(&config, &repository, generation, &working)?
    };
    let height = applied_state_commit(&generation.state).target.height;
    let markers = build_applied_markers(&generation.state, height)?;
    let publication = Arc::new(RwLock::new(Arc::new(AppliedPublication {
        generation,
        markers,
        durable_cursor: cursor,
        admitted_cursor: cursor,
        storage_failed: false,
        segmented_position: None,
    })));
    let reader = AppliedReader {
        storage,
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
        effective_storage_identity,
        backend: crate::sync::applied::types::AppliedBackend::Compact {
            worker: Some(worker),
            pool,
        },
        reader: reader.clone(),
        pending,
        checkpoint: None,
        admitted_cursor: cursor,
        durable_cursor: cursor,
        database_sequence: 0,
        storage_failed: false,
        metadata_lease,
    };
    Ok((owner, reader))
}
