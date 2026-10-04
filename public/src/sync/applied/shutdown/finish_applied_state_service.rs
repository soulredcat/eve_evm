// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    persistence::worker::{RecordWorkerError, finish_record_worker},
    sync::applied::{
        AppliedOwner, AppliedShutdown, RetainedAppliedTail, capture_applied_state,
        poll_applied_durability, types::AppliedBackend,
    },
};

/// Drain/join outside publication locks, then reconcile tickets and return every unacknowledged byte.
pub fn finish_applied_state_service(mut owner: AppliedOwner) -> AppliedShutdown {
    let mut segmented_tails = None;
    let repository = match &mut owner.backend {
        AppliedBackend::Compact { worker, .. } => match worker.take() {
            Some(worker) => finish_record_worker(worker),
            None => Err(RecordWorkerError::Closed),
        },
        AppliedBackend::Segmented { worker, .. } => match worker.take() {
            Some(worker) => {
                let shutdown = crate::persistence::segmented::finish_segmented_worker(worker);
                segmented_tails = Some(shutdown.tails);
                shutdown.repository.map_err(|error| match error {
                    crate::persistence::segmented::SegmentedError::WorkerPanicked => {
                        RecordWorkerError::WorkerPanicked
                    }
                    _ => RecordWorkerError::StorageFailed,
                })
            }
            None => Err(RecordWorkerError::Closed),
        },
    };
    let acknowledgement_error = poll_applied_durability(&mut owner).err();
    let publication = capture_applied_state(&owner.reader).ok();
    if let Some(tails) = &mut segmented_tails {
        for tail in tails.iter_mut() {
            if tail
                .as_ref()
                .and_then(|tail| tail.complete_marker)
                .is_some_and(|ack| {
                    ack.marker_cursor.sequence <= owner.durable_cursor.sequence
                        && !owner
                            .pending
                            .iter()
                            .any(|pending| pending.target_cursor == ack.marker_cursor)
                })
            {
                // Complete marker already passed this owner's ordered prefix validation.
                *tail = None;
            }
        }
    }
    AppliedShutdown {
        repository,
        acknowledgement_error,
        unacknowledged_tail: RetainedAppliedTail {
            pending: owner.pending,
            _metadata_lease: owner.metadata_lease,
            segmented_tails,
        },
        publication,
    }
}
