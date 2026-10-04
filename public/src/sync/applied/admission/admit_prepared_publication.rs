// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    persistence::{
        handoff::{RecoveryPayload, observe_handoff},
        worker::try_submit_record,
    },
    sync::applied::{
        AppliedAdmission, AppliedError, AppliedOwner, AppliedPublication,
        types::{AppliedBackend, PendingPayload, PendingRecord},
    },
};
use eve_state::StateVersion;
use eve_storage::records::OpaqueRecordCursor;
use std::{sync::Arc, time::Duration};

/// Under the short publication guard, perform only immediate RAM admission and pointer updates.
/// Candidate allocation and all replay/proof work precede this operation. The worker's
/// bounded reply-channel/ID bookkeeping may allocate here; no disk or network wait occurs.
pub(in crate::sync::applied) fn admit_prepared_publication(
    owner: &mut AppliedOwner,
    publication: Arc<AppliedPublication>,
    payload: RecoveryPayload,
    target_cursor: OpaqueRecordCursor,
    target: StateVersion,
) -> Result<AppliedAdmission, AppliedError> {
    let AppliedBackend::Compact { worker, pool } = &owner.backend else {
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
    let handoff = observe_handoff(pool).map_err(AppliedError::Handoff)?;
    if handoff.oldest_age > Duration::from_millis(owner.config.public_budget.queue_age_ms) {
        return Err(AppliedError::QueueLimit);
    }
    // The pending clone retains this same charged buffer even if admitted storage later fails.
    let ticket = try_submit_record(worker, owner.admitted_cursor, payload.clone())
        .map_err(|rejected| AppliedError::Worker(rejected.error))?;
    owner.pending.push_back(PendingRecord {
        parent: owner.admitted_cursor,
        target_cursor,
        target,
        payload: PendingPayload::Compact { ticket, payload },
    });
    let applied = publication.markers.applied;
    let previous = std::mem::replace(&mut *current, publication);
    owner.admitted_cursor = target_cursor;
    drop(current);
    drop(previous);
    Ok(AppliedAdmission {
        applied,
        admitted_cursor: target_cursor,
    })
}
