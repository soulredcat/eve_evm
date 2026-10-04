// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    AppliedError, AppliedMode, AppliedOwner, AppliedPublication, AppliedReader,
    opening::initialize_charged_generation::initialize_charged_generation,
    publication::build_applied_markers,
    resources::{
        create_estimated_working_pool, estimate_pending_metadata, estimated_repository_read_charge,
        reserve_estimated_working,
    },
    types::AppliedBackend,
};
use super::{
    SegmentedAppliedConfig, recover_segmented_applied_prefix::recover_segmented_applied_prefix,
};
use crate::persistence::segmented::{create_segmented_part_pool, start_segmented_worker};
use eve_finality_verifier::MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES;
use eve_node_policy::validate_public_budget;
use eve_state::DevelopmentGenesis;
use eve_storage::records::{opaque_record_cursor, open_opaque_record_repository};
use sha2::{Digest, Sha256};
use std::{
    collections::VecDeque,
    sync::{Arc, RwLock},
};

/// Explicit V2 namespace/profile, preserving compact configuration and constructors.
pub fn open_segmented_applied_state_service(
    config: SegmentedAppliedConfig,
    genesis: &DevelopmentGenesis,
) -> Result<(AppliedOwner, AppliedReader), AppliedError> {
    let application = config.application;
    validate_public_budget(application.public_budget)
        .map_err(|_| AppliedError::InvalidConfiguration)?;
    if config.codec.maximum_logical_bytes > MAXIMUM_LOGICAL_IMPORT_WIRE_BYTES
        || application.worker_scratch_limit != config.policy.maximum_scratch_bytes
    {
        return Err(AppliedError::InvalidConfiguration);
    }
    let working =
        create_estimated_working_pool(application.public_budget.maximum_working_state_bytes)?;
    let generation = initialize_charged_generation(
        &application,
        genesis,
        AppliedMode::AuthenticatedImport,
        &working,
    )?;
    if super::super::state::applied_state_commit(&generation.state)
        .target
        .identity
        .genesis
        .0
        .0
        != application.identity.genesis_hash
    {
        return Err(AppliedError::InvalidConfiguration);
    }
    let mut effective_storage_identity = application.identity;
    let mut domain = Sha256::new();
    domain.update(b"EVE_PUBLIC_SEGMENTED_IMPORT_DOMAIN_V2");
    domain.update(effective_storage_identity.domain);
    effective_storage_identity.domain = domain.finalize().into();
    let pool = create_segmented_part_pool(
        application.public_budget,
        config.policy,
        config.codec,
        application.repository_budget,
        effective_storage_identity,
    )
    .map_err(AppliedError::Segmented)?;
    let read = reserve_estimated_working(
        &working,
        estimated_repository_read_charge(&application.repository_budget)?,
    )?;
    let repository = open_opaque_record_repository(
        &application.path,
        effective_storage_identity,
        application.repository_budget,
    )
    .map_err(|_| AppliedError::StorageUnavailable)?;
    drop(read);
    let (generation, position) = recover_segmented_applied_prefix(
        &application,
        &repository,
        &config.codec,
        generation,
        &working,
    )?;
    let physical =
        opaque_record_cursor(&repository).map_err(|_| AppliedError::StorageUnavailable)?;
    let markers = build_applied_markers(&generation.state, position.durable.height)?;
    let reader = AppliedReader {
        publication: Arc::new(RwLock::new(Arc::new(AppliedPublication {
            generation,
            markers,
            durable_cursor: position.durable.cursor,
            admitted_cursor: physical,
            storage_failed: false,
            segmented_position: Some(position),
        }))),
        working,
    };
    let capacity = usize::try_from(config.policy.retained_parts / 2)
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
    let worker = start_segmented_worker(repository, Arc::clone(&pool), position.durable)
        .map_err(AppliedError::Segmented)?;
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
        admitted_cursor: physical,
        durable_cursor: position.durable.cursor,
        database_sequence: 0,
        storage_failed: false,
        metadata_lease,
    };
    Ok((owner, reader))
}
