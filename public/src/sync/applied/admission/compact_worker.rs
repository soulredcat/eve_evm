// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    persistence::worker::RecordWorker,
    sync::applied::{AppliedError, AppliedOwner, types::AppliedBackend},
};
pub(in crate::sync::applied) fn compact_worker(
    owner: &AppliedOwner,
) -> Result<&RecordWorker, AppliedError> {
    match &owner.backend {
        AppliedBackend::Compact { worker, .. } => worker.as_ref().ok_or(AppliedError::Closed),
        AppliedBackend::Segmented { .. } => Err(AppliedError::WrongMode),
    }
}
