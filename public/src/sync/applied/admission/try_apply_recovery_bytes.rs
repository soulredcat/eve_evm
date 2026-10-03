// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::admit_prepared_publication::admit_prepared_publication;
use crate::{
    persistence::{
        handoff::{
            observe_handoff, recovery_payload_bytes, reserve_recovery_payload,
            seal_recovery_payload, write_reserved_payload,
        },
        worker::observe_record_worker,
    },
    sync::applied::{
        AppliedAdmission, AppliedError, AppliedOwner, AppliedPublication,
        publication::{build_applied_markers, capture_applied_state},
        recovery::prepare_empty_recovery,
        resources::{estimate_replay_charge, reserve_estimated_working, split_estimated_working},
        types::ChargedRecoveryState,
    },
};
use eve_evm::estimate_clone_reservation;
use eve_finality_verifier::{
    into_recovery_state, recovery_state_commit, validate_empty_recovery_envelope_bytes,
};
use eve_storage::records::prospective_opaque_record_cursor;
use std::{sync::Arc, time::Duration};

/// Temporary empty-execution/empty-lookahead capability; nonempty records remain unsupported here.
/// Borrowed ingress bytes need their caller's separate input reservation.
pub fn try_apply_recovery_bytes(
    owner: &mut AppliedOwner,
    bytes: &[u8],
) -> Result<AppliedAdmission, AppliedError> {
    if owner.storage_failed {
        return Err(AppliedError::StorageFailed);
    }
    let worker = owner.worker.as_ref().ok_or(AppliedError::Closed)?;
    if observe_record_worker(worker).storage_failed {
        return Err(AppliedError::StorageFailed);
    }
    if bytes.is_empty() || bytes.len() > owner.config.maximum_recovery_payload_bytes {
        return Err(AppliedError::PayloadLimit);
    }
    validate_empty_recovery_envelope_bytes(bytes).map_err(AppliedError::Recovery)?;
    let handoff = observe_handoff(&owner.pool).map_err(AppliedError::Handoff)?;
    if owner.pending.len() as u64 >= owner.config.public_budget.queue_batches
        || handoff.oldest_age > Duration::from_millis(owner.config.public_budget.queue_age_ms)
    {
        return Err(AppliedError::QueueLimit);
    }
    let parent = capture_applied_state(&owner.reader)?;
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
    let oracle =
        estimate_clone_reservation(&recovery_state_commit(&parent.generation.recovery).state)
            .map_err(|_| AppliedError::EstimatedCapacity)?;
    let charge = estimate_replay_charge(
        &owner.config.state_budget,
        owner.config.maximum_recovery_payload_bytes,
        oracle,
    )?;
    let lease = reserve_estimated_working(&owner.reader.working, charge.total)?;
    let mut buffer =
        reserve_recovery_payload(&owner.pool, bytes.len()).map_err(AppliedError::Handoff)?;
    write_reserved_payload(&mut buffer, bytes).map_err(AppliedError::Handoff)?;
    let payload = seal_recovery_payload(buffer).map_err(AppliedError::Handoff)?;
    let transition = prepare_empty_recovery(
        &parent.generation,
        recovery_payload_bytes(&payload),
        &owner.config.state_budget,
        oracle,
    )?;
    let recovery = into_recovery_state(transition);
    let target = recovery_state_commit(&recovery).target.clone();
    let markers = build_applied_markers(&recovery, parent.markers.durable_recovery.0)?;
    let cursor = prospective_opaque_record_cursor(
        owner.config.identity,
        owner.admitted_cursor,
        recovery_payload_bytes(&payload),
        owner.config.repository_budget.maximum_record_bytes,
    )
    .map_err(|_| AppliedError::InvalidDurablePrefix)?;
    let (retained, transient) = split_estimated_working(lease, charge.retained)?;
    let generation = Arc::new(ChargedRecoveryState {
        recovery,
        _lease: retained,
    });
    let publication = Arc::new(AppliedPublication {
        generation,
        markers,
        durable_cursor: owner.durable_cursor,
        admitted_cursor: cursor,
        storage_failed: false,
    });
    drop(transient);
    admit_prepared_publication(owner, publication, payload, cursor, target)
}
