// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    AppliedAdmission, AppliedError, AppliedOwner, AppliedPublication,
    types::{AppliedBackend, PendingPayload, PendingRecord},
};
use crate::persistence::segmented::{SealedSegmentedBatch, try_submit_segmented_batch};
use eve_state::StateVersion;
use eve_storage::records::segmented::SegmentedLogicalIdentity;
use std::sync::Arc;

pub(super) fn admit_segmented_publication(
    owner: &mut AppliedOwner,
    publication: Arc<AppliedPublication>,
    payload: SealedSegmentedBatch,
    identity: SegmentedLogicalIdentity,
    target_binding: [u8; 32],
    target: StateVersion,
) -> Result<AppliedAdmission, AppliedError> {
    let AppliedBackend::Segmented { worker, .. } = &owner.backend else {
        return Err(AppliedError::WrongMode);
    };
    let worker = worker.as_ref().ok_or(AppliedError::Closed)?;
    let mut current = owner
        .reader
        .publication
        .write()
        .map_err(|_| AppliedError::PublicationUnavailable)?;
    if owner.pending.len() == owner.pending.capacity() {
        return Err(AppliedError::QueueLimit);
    }
    let ticket = try_submit_segmented_batch(worker, owner.admitted_cursor, payload.clone())
        .map_err(|rejected| AppliedError::Segmented(rejected.error))?;
    let cursor = publication.admitted_cursor;
    let applied = publication.markers.applied;
    owner.pending.push_back(PendingRecord {
        parent: identity.parent.cursor,
        target_cursor: cursor,
        target,
        payload: PendingPayload::Segmented {
            ticket,
            payload,
            identity,
            target_binding,
        },
    });
    let previous = std::mem::replace(&mut *current, publication);
    owner.admitted_cursor = cursor;
    drop(current);
    drop(previous);
    Ok(AppliedAdmission {
        applied,
        admitted_cursor: cursor,
    })
}
