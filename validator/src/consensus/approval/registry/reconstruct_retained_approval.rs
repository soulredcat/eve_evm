// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ApprovalRegistry;
use crate::consensus::{
    approval::{ApprovalError, ExecutionApproval, create_execution_approval},
    signing::DurableSigner,
};
use eve_state::StateBudget;
use std::sync::Arc;

/// Borrow retained sealed data after dropping cache lock. This is not fresh unlocked evidence.
pub(in crate::consensus) fn reconstruct_retained_approval(
    registry: &ApprovalRegistry,
    signer: &DurableSigner,
    hash: &[u8; 32],
    budget: &StateBudget,
    reserved_clone_bytes: usize,
) -> Result<Arc<ExecutionApproval>, ApprovalError> {
    let raw = {
        let state = registry
            .state
            .lock()
            .map_err(|_| ApprovalError::Unavailable {
                reason: "approval registry poisoned",
                cause: None,
            })?;
        state
            .raw
            .get(hash)
            .cloned()
            .ok_or(ApprovalError::Unavailable {
                reason: "full proposal data unavailable",
                cause: None,
            })?
    };
    Ok(Arc::new(create_execution_approval(
        signer,
        &raw.source,
        budget,
        reserved_clone_bytes,
    )?))
}
