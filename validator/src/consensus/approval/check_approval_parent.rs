// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ApprovalError, ExecutionApproval};
use crate::consensus::signing::{DurableSigner, signer_status};
use eve_storage::state::read_state_service;

pub(super) fn check_approval_parent(
    approval: &ExecutionApproval,
    signer: &DurableSigner,
) -> Result<(), ApprovalError> {
    let unavailable = |reason| ApprovalError::Unavailable {
        reason,
        cause: None,
    };
    if signer_status(signer).fenced || approval.config != signer.config {
        return Err(unavailable("approval signer context unavailable"));
    }
    let current = read_state_service(&signer.service)
        .map_err(|_| unavailable("approval state service unavailable"))?;
    if current.sequence() != approval.database_sequence
        || current.commit().target != approval.parent
    {
        return Err(unavailable("approval parent is stale"));
    }
    Ok(())
}
