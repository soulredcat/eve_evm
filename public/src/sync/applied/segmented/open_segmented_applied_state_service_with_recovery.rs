// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    SegmentedAppliedConfig, derive_segmented_storage_identity::derive_segmented_storage_identity,
};
use crate::{
    persistence::segmented::create_segmented_part_pool,
    sync::applied::{
        AppliedError, AppliedMode, AppliedOwner, AppliedPublication, AppliedReader,
        checkpoints::{
            CheckpointAppliedError, CheckpointRecoveryConfig,
            recovery::{recover_checkpoint_applied_prefix, start_recovered_segmented_worker},
        },
        opening::initialize_charged_generation::initialize_charged_generation,
        publication::build_applied_markers,
        resources::{
            create_estimated_working_pool, estimate_pending_metadata,
            estimated_repository_read_charge, reserve_estimated_working,
        },
        state::applied_state_commit,
        types::AppliedBackend,
    },
};
use eve_finality_verifier::MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES;
use eve_node_policy::{CheckpointHeight, validate_public_budget, validate_watermarks};
use eve_state::DevelopmentGenesis;
use eve_storage::records::{opaque_record_cursor, open_opaque_record_repository};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

/// One startup composition for legacy genesis recovery and explicit checkpoint-aware recovery.
pub(in crate::sync::applied) fn open_segmented_applied_state_service_with_recovery(
    config: SegmentedAppliedConfig,
    recovery: Option<CheckpointRecoveryConfig>,
    genesis: &DevelopmentGenesis,
) -> Result<(AppliedOwner, AppliedReader), CheckpointAppliedError> {
    let application = config.application;
    validate_public_budget(application.public_budget)
        .map_err(|_| CheckpointAppliedError::Applied(AppliedError::InvalidConfiguration))?;
    if config.codec.maximum_logical_bytes > MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES
        || application.worker_scratch_limit != config.policy.maximum_scratch_bytes
        || recovery.as_ref().is_some_and(|limits| {
            limits.maximum_scan_records == 0
                || limits.maximum_scan_records
                    > application.repository_budget.maximum_retained_records
        })
    {
        return Err(CheckpointAppliedError::Applied(
            AppliedError::InvalidConfiguration,
        ));
    }
    let working =
        create_estimated_working_pool(application.public_budget.maximum_working_state_bytes)
            .map_err(CheckpointAppliedError::Applied)?;
    let generation = initialize_charged_generation(
        &application,
        genesis,
        AppliedMode::AuthenticatedImport,
        &working,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let storage = super::super::resources::storage_admission::create_storage_admission_pool(
        application.public_budget,
        &working,
    )
    .map_err(super::super::resources::storage_admission::map_storage_admission_error)
    .map_err(CheckpointAppliedError::Applied)?;
    let resource_pools = super::super::resources::storage_admission::AppliedStorageResourcePools {
        working: Arc::clone(&working),
        storage: Arc::clone(&storage),
    };
    if applied_state_commit(&generation.state)
        .target
        .identity
        .genesis
        .0
        .0
        != application.identity.genesis_hash
    {
        return Err(CheckpointAppliedError::Applied(
            AppliedError::InvalidConfiguration,
        ));
    }
    let effective_storage_identity = derive_segmented_storage_identity(application.identity);
    let pool = create_segmented_part_pool(
        application.public_budget,
        config.policy,
        config.codec,
        application.repository_budget,
        effective_storage_identity,
    )
    .map_err(CheckpointAppliedError::Segmented)?;
    let read = reserve_estimated_working(
        &working,
        estimated_repository_read_charge(&application.repository_budget)
            .map_err(CheckpointAppliedError::Applied)?,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let _startup_read = super::super::resources::storage_admission::reserve_storage_read(&storage)
        .map_err(super::super::resources::storage_admission::map_storage_admission_error)
        .map_err(CheckpointAppliedError::Applied)?;
    let repository = open_opaque_record_repository(
        &application.path,
        effective_storage_identity,
        application.repository_budget,
    )
    .map_err(|_| CheckpointAppliedError::Applied(AppliedError::StorageUnavailable))?;
    drop(_startup_read);
    drop(read);
    let recovered = recover_checkpoint_applied_prefix(
        &application,
        recovery.as_ref(),
        &repository,
        &config.codec,
        generation,
        &resource_pools,
        genesis,
    )?;
    let physical = opaque_record_cursor(&repository)
        .map_err(|_| CheckpointAppliedError::Applied(AppliedError::StorageUnavailable))?;
    let mut markers = build_applied_markers(
        &recovered.generation.state,
        recovered.position.durable.height,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    markers.checkpoint = CheckpointHeight(recovered.checkpoint_height);
    markers.authenticated_snapshot_height = recovered.checkpoint_height;
    validate_watermarks(markers).map_err(|_| CheckpointAppliedError::InvalidArtifactBinding)?;
    let reader = AppliedReader {
        storage,
        publication: Arc::new(RwLock::new(Arc::new(AppliedPublication {
            generation: Arc::clone(&recovered.generation),
            markers,
            durable_cursor: recovered.position.durable.cursor,
            admitted_cursor: physical,
            storage_failed: false,
            segmented_position: Some(recovered.position),
        }))),
        working,
    };
    let capacity = usize::try_from(config.policy.retained_parts / 2)
        .map_err(|_| CheckpointAppliedError::Applied(AppliedError::InvalidConfiguration))?;
    let metadata_lease = reserve_estimated_working(
        &reader.working,
        estimate_pending_metadata(capacity).map_err(CheckpointAppliedError::Applied)?,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let mut pending = VecDeque::new();
    pending
        .try_reserve_exact(capacity)
        .map_err(|_| CheckpointAppliedError::Applied(AppliedError::AllocationFailed))?;
    if pending.capacity() != capacity {
        return Err(CheckpointAppliedError::Applied(
            AppliedError::AllocationFailed,
        ));
    }
    let worker = start_recovered_segmented_worker(repository, Arc::clone(&pool), &recovered)?;
    let owner = AppliedOwner {
        config: application,
        effective_storage_identity,
        backend: AppliedBackend::Segmented {
            worker: Some(worker),
            pool,
            codec: config.codec,
        },
        reader: reader.clone(),
        pending,
        checkpoint: None,
        admitted_cursor: physical,
        durable_cursor: recovered.position.durable.cursor,
        database_sequence: 0,
        storage_failed: false,
        metadata_lease,
    };
    Ok((owner, reader))
}
