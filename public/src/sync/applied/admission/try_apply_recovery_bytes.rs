// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{admit_prepared_publication::admit_prepared_publication, compact_pool, compact_worker};
use crate::{
    persistence::{
        handoff::{
            observe_handoff, recovery_payload_bytes, reserve_recovery_payload,
            seal_recovery_payload, write_reserved_payload,
        },
        worker::observe_record_worker,
    },
    sync::applied::{
        AppliedAdmission, AppliedError, AppliedMode, AppliedOwner, AppliedPublication,
        publication::{applied_mode, build_next_applied_markers, capture_applied_state},
        recovery::prepare_charged_generation,
        state::applied_state_commit,
    },
};
use eve_finality_verifier::{
    preflight_authenticated_import_wire, validate_empty_recovery_envelope_bytes,
};
use eve_storage::records::prospective_opaque_record_cursor;
use std::{sync::Arc, time::Duration};

/// Prepare according to the explicit stored mode; caller ingress bytes need a separate charge.
pub fn try_apply_recovery_bytes(
    owner: &mut AppliedOwner,
    bytes: &[u8],
) -> Result<AppliedAdmission, AppliedError> {
    if owner.checkpoint.is_some() {
        return Err(AppliedError::CheckpointPending);
    }
    if matches!(
        &owner.backend,
        crate::sync::applied::types::AppliedBackend::Segmented { .. }
    ) {
        return crate::sync::applied::segmented::try_apply_segmented_import(owner, bytes, None);
    }
    if owner.storage_failed {
        return Err(AppliedError::StorageFailed);
    }
    let worker = compact_worker(owner)?;
    if observe_record_worker(worker).storage_failed {
        return Err(AppliedError::StorageFailed);
    }
    if bytes.is_empty() || bytes.len() > owner.config.maximum_recovery_payload_bytes {
        return Err(AppliedError::PayloadLimit);
    }
    let parent = capture_applied_state(&owner.reader)?;
    match applied_mode(&parent) {
        AppliedMode::EmptyReplay => {
            validate_empty_recovery_envelope_bytes(bytes).map_err(AppliedError::Recovery)?
        }
        AppliedMode::AuthenticatedImport => {
            preflight_authenticated_import_wire(bytes, &owner.config.state_budget)
                .map_err(AppliedError::ImportWire)?;
        }
    }
    let handoff = observe_handoff(compact_pool(owner)?).map_err(AppliedError::Handoff)?;
    if owner.pending.len() as u64 >= owner.config.public_budget.queue_batches
        || handoff.oldest_age > Duration::from_millis(owner.config.public_budget.queue_age_ms)
    {
        return Err(AppliedError::QueueLimit);
    }
    let height = parent
        .markers
        .applied
        .0
        .checked_add(1)
        .ok_or(AppliedError::ArithmeticOverflow)?;
    if height.saturating_sub(parent.markers.durable_recovery.0)
        > owner.config.public_budget.maximum_durable_lag_blocks
    {
        return Err(AppliedError::QueueLimit);
    }
    let mut buffer = reserve_recovery_payload(compact_pool(owner)?, bytes.len())
        .map_err(AppliedError::Handoff)?;
    write_reserved_payload(&mut buffer, bytes).map_err(AppliedError::Handoff)?;
    let payload = seal_recovery_payload(buffer).map_err(AppliedError::Handoff)?;
    let generation = prepare_charged_generation(
        &parent.generation,
        recovery_payload_bytes(&payload),
        &owner.config,
        &owner.reader.working,
    )?;
    let target = applied_state_commit(&generation.state).target.clone();
    let markers = build_next_applied_markers(
        &generation.state,
        parent.markers.durable_recovery.0,
        &parent,
    )?;
    let cursor = prospective_opaque_record_cursor(
        owner.effective_storage_identity,
        owner.admitted_cursor,
        recovery_payload_bytes(&payload),
        owner.config.repository_budget.maximum_record_bytes,
    )
    .map_err(|_| AppliedError::InvalidDurablePrefix)?;
    let publication = Arc::new(AppliedPublication {
        generation,
        markers,
        durable_cursor: owner.durable_cursor,
        admitted_cursor: cursor,
        storage_failed: false,
        segmented_position: None,
    });
    admit_prepared_publication(owner, publication, payload, cursor, target)
}
