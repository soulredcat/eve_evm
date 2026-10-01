// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ApprovalRegistry;
use crate::consensus::approval::{ApprovalError, ExecutionApproval};
use std::sync::Arc;

pub(in crate::consensus) fn find_approval(
    registry: &ApprovalRegistry,
    hash: &[u8; 32],
) -> Result<Option<Arc<ExecutionApproval>>, ApprovalError> {
    let state = registry
        .state
        .lock()
        .map_err(|_| ApprovalError::Unavailable {
            reason: "approval registry poisoned",
            cause: None,
        })?;
    Ok(state.full.get(hash).cloned())
}
