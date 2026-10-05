// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    persistence::worker::{RecordWorkerError, finish_record_worker},
    sync::applied::{
        AppliedOwner, AppliedShutdown, RetainedAppliedTail, capture_applied_state,
        poll_applied_durability,
    },
};

/// Drain/join outside publication locks, then reconcile tickets and return every unacknowledged byte.
pub fn finish_applied_state_service(mut owner: AppliedOwner) -> AppliedShutdown {
    let repository = match owner.worker.take() {
        Some(worker) => finish_record_worker(worker),
        None => Err(RecordWorkerError::Closed),
    };
    let acknowledgement_error = poll_applied_durability(&mut owner).err();
    let publication = capture_applied_state(&owner.reader).ok();
    AppliedShutdown {
        repository,
        acknowledgement_error,
        unacknowledged_tail: RetainedAppliedTail {
            pending: owner.pending,
            _metadata_lease: owner.metadata_lease,
        },
        publication,
    }
}
