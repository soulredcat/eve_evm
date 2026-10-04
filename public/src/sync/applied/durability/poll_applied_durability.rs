// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::validate_applied_acknowledgement::validate_applied_acknowledgement;
use crate::{
    persistence::worker::try_receive_record_ack,
    sync::applied::{
        AppliedError, AppliedOwner, AppliedPublication, capture_applied_state,
        resources::{estimate_pending_metadata, reserve_estimated_working},
        types::PendingPayload,
    },
};
use eve_node_policy::PublicWatermarks;
use std::{collections::VecDeque, sync::Arc};

/// Poll only the ordered front. A pending ticket never advances the durable marker.
pub fn poll_applied_durability(owner: &mut AppliedOwner) -> Result<PublicWatermarks, AppliedError> {
    if owner.checkpoint.is_some() {
        return Err(AppliedError::CheckpointPending);
    }
    if owner.storage_failed {
        return Err(AppliedError::StorageFailed);
    }
    let current = capture_applied_state(&owner.reader)?;
    if owner.pending.is_empty() {
        return Ok(current.markers);
    }
    let metadata_lease = reserve_estimated_working(
        &owner.reader.working,
        estimate_pending_metadata(owner.pending.len())?,
    )?;
    let mut retired = VecDeque::new();
    retired
        .try_reserve_exact(owner.pending.len())
        .map_err(|_| AppliedError::AllocationFailed)?;
    if retired.capacity() != owner.pending.len() {
        return Err(AppliedError::AllocationFailed);
    }
    // Allocate publication bookkeeping before locking; its generation shares the original lease.
    let mut next = Arc::new(AppliedPublication {
        generation: Arc::clone(&current.generation),
        markers: current.markers,
        durable_cursor: owner.durable_cursor,
        admitted_cursor: owner.admitted_cursor,
        storage_failed: false,
        segmented_position: current.segmented_position,
    });
    let fields = Arc::get_mut(&mut next).ok_or(AppliedError::PublicationUnavailable)?;
    // Lock before consuming acknowledgements: poison rejection cannot lose an unprocessed ticket.
    let mut publication = owner
        .reader
        .publication
        .write()
        .map_err(|_| AppliedError::PublicationUnavailable)?;
    let mut markers = current.markers;
    let mut error = None;
    while let Some(pending) = owner.pending.front() {
        let (cursor, sequence, segmented_anchor) = match &pending.payload {
            PendingPayload::Compact { ticket, .. } => {
                let ack = match try_receive_record_ack(ticket) {
                    Ok(None) => break,
                    Ok(Some(ack)) => ack,
                    Err(worker_error) => {
                        error = Some(AppliedError::Worker(worker_error));
                        break;
                    }
                };
                if let Err(failure) = validate_applied_acknowledgement(
                    pending,
                    ack,
                    owner.durable_cursor,
                    markers.durable_recovery.0,
                    owner.database_sequence,
                ) {
                    error = Some(failure);
                    break;
                }
                (ack.appended, ack.database_sequence, None)
            }
            PendingPayload::Segmented { ticket, .. } => {
                let ack = match crate::persistence::segmented::try_receive_segmented_ack(ticket) {
                    Ok(None) => break,
                    Ok(Some(ack)) => ack,
                    Err(worker_error) => {
                        error = Some(AppliedError::Segmented(worker_error));
                        break;
                    }
                };
                if let Err(failure) =
                    crate::sync::applied::segmented::validate_segmented_applied_ack(
                        pending,
                        ack,
                        owner.durable_cursor,
                        markers.durable_recovery.0,
                        owner.database_sequence,
                    )
                {
                    error = Some(failure);
                    break;
                }
                let anchor = eve_storage::records::segmented::SegmentedRecoveryAnchor {
                    height: ack.identity.target_height,
                    cursor: ack.marker_cursor,
                    state_binding: ack.target_state_binding,
                };
                (ack.marker_cursor, ack.database_sequence, Some(anchor))
            }
        };
        markers.durable_recovery.0 = pending.target.height;
        owner.durable_cursor = cursor;
        owner.database_sequence = sequence;
        if let (Some(position), Some(anchor)) = (&mut fields.segmented_position, segmented_anchor) {
            position.durable = anchor;
            position.last_acknowledged_physical = cursor;
            if position
                .missing_from
                .is_some_and(|height| height <= anchor.height)
            {
                position.missing_from = None;
            }
        }
        if let Some(record) = owner.pending.pop_front() {
            retired.push_back(record);
        }
    }
    owner.storage_failed = error.is_some();
    fields.markers = markers;
    fields.durable_cursor = owner.durable_cursor;
    fields.storage_failed = owner.storage_failed;
    if markers == current.markers && error.is_none() {
        drop(publication);
        return Ok(markers);
    }
    let previous = std::mem::replace(&mut *publication, next);
    drop(publication);
    drop(previous);
    drop(retired);
    drop(metadata_lease);
    match error {
        Some(error) => Err(error),
        None => Ok(markers),
    }
}
