// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    persistence::handoff::HandoffPool,
    sync::applied::{AppliedError, AppliedOwner, types::AppliedBackend},
};
use std::sync::Arc;
pub(in crate::sync::applied) fn compact_pool(
    owner: &AppliedOwner,
) -> Result<&Arc<HandoffPool>, AppliedError> {
    match &owner.backend {
        AppliedBackend::Compact { pool, .. } => Ok(pool),
        AppliedBackend::Segmented { .. } => Err(AppliedError::WrongMode),
    }
}
