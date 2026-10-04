// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    AppliedAdmission, AppliedError, AppliedOwner, AppliedPublication,
    publication::{build_next_applied_markers, capture_applied_state},
    recovery::prepare_charged_import,
    resources::reserve_estimated_working,
    state::{AppliedState, applied_state_commit},
    types::AppliedBackend,
};
use super::{admit_segmented_publication::admit_segmented_publication, bind_local_state_version};
use crate::persistence::segmented::{
    bind_segmented_reservation, observe_segmented_parts, observe_segmented_worker,
    plan_segmented_layout, reserve_segmented_layout, segmented_batch_marker_cursor,
    write_and_seal_segmented_batch,
};
use eve_finality_verifier::preflight_logical_import_wire;
use eve_state::{BOUNDED_STATE_CODEC_SCRATCH_BYTES, StateVersion};
use eve_storage::records::segmented::{
    SegmentedLogicalIdentity, SegmentedRecoveryAnchor, SegmentedRecoveryMode,
    hash_segmented_logical_body,
};
use std::{sync::Arc, time::Duration};

pub(in crate::sync::applied) fn try_apply_segmented_import(
    owner: &mut AppliedOwner,
    bytes: &[u8],
    expected_target: Option<&StateVersion>,
) -> Result<AppliedAdmission, AppliedError> {
    if owner.checkpoint.is_some() {
        return Err(AppliedError::CheckpointPending);
    }
    if owner.storage_failed {
        return Err(AppliedError::StorageFailed);
    }
    let AppliedBackend::Segmented {
        worker,
        pool,
        codec,
    } = &owner.backend
    else {
        return Err(AppliedError::WrongMode);
    };
    let worker = worker.as_ref().ok_or(AppliedError::Closed)?;
    if observe_segmented_worker(worker)
        .map_err(AppliedError::Segmented)?
        .storage_failed
    {
        return Err(AppliedError::StorageFailed);
    }
    if bytes.is_empty() || bytes.len() > codec.maximum_logical_bytes {
        return Err(AppliedError::PayloadLimit);
    }
    preflight_logical_import_wire(bytes, &owner.config.state_budget)
        .map_err(AppliedError::ImportWire)?;
    let parent = capture_applied_state(&owner.reader)?;
    let position = parent.segmented_position.ok_or(AppliedError::WrongMode)?;
    let next_height = position
        .applied
        .height
        .checked_add(1)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    if next_height.saturating_sub(position.durable.height)
        > owner.config.public_budget.maximum_durable_lag_blocks
    {
        return Err(AppliedError::QueueLimit);
    }
    let observation = observe_segmented_parts(pool).map_err(AppliedError::Segmented)?;
    if observation.oldest_age > Duration::from_millis(owner.config.public_budget.queue_age_ms)
        || owner.pending.len() == owner.pending.capacity()
    {
        return Err(AppliedError::QueueLimit);
    }
    let identity = SegmentedLogicalIdentity {
        mode: SegmentedRecoveryMode::AuthenticatedImport,
        logical_id: hash_segmented_logical_body(bytes),
        parent: position.applied,
        target_height: next_height,
        total_length: bytes.len() as u64,
    };
    let layout = plan_segmented_layout(pool, identity).map_err(AppliedError::Segmented)?;
    let unbound = reserve_segmented_layout(pool, &layout).map_err(AppliedError::Segmented)?;
    let AppliedState::AuthenticatedImport(imported) = &parent.generation.state else {
        return Err(AppliedError::WrongMode);
    };
    let generation = prepare_charged_import(
        imported,
        bytes,
        &owner.config.state_budget,
        &owner.reader.working,
        true,
    )?;
    let sizing =
        reserve_estimated_working(&owner.reader.working, BOUNDED_STATE_CODEC_SCRATCH_BYTES)?;
    let target = applied_state_commit(&generation.state).target.clone();
    if expected_target.is_some_and(|expected| expected != &target) {
        return Err(AppliedError::InvalidDurablePrefix);
    }
    let binding = bind_local_state_version(&target)?;
    let bound = bind_segmented_reservation(unbound, binding)
        .map_err(|rejected| AppliedError::Segmented(rejected.error))?;
    let batch = write_and_seal_segmented_batch(bound, bytes, owner.admitted_cursor)
        .map_err(AppliedError::Segmented)?;
    let marker = segmented_batch_marker_cursor(&batch);
    let mut next_position = position;
    next_position.applied = SegmentedRecoveryAnchor {
        height: next_height,
        cursor: marker,
        state_binding: binding,
    };
    let markers = build_next_applied_markers(&generation.state, position.durable.height, &parent)?;
    let publication = Arc::new(AppliedPublication {
        generation,
        markers,
        admitted_cursor: marker,
        durable_cursor: position.durable.cursor,
        storage_failed: false,
        segmented_position: Some(next_position),
    });
    drop(sizing);
    admit_segmented_publication(owner, publication, batch, identity, binding, target)
}
