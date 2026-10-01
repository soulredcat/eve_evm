// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ApprovalRegistry, RegistryState};
use crate::consensus::{approval::ApprovalError, signing::DurableSigner};
use eve_storage::state::read_state_service;

pub(in crate::consensus) fn clear_after_synced_commit(
    registry: &ApprovalRegistry,
    signer: &DurableSigner,
) -> Result<(), ApprovalError> {
    let unavailable = |reason| ApprovalError::Unavailable {
        reason,
        cause: None,
    };
    let current = read_state_service(&signer.service)
        .map_err(|_| unavailable("committed state unavailable"))?;
    let mut state = registry
        .state
        .lock()
        .map_err(|_| unavailable("approval registry poisoned"))?;
    if state
        .config
        .as_ref()
        .is_some_and(|config| config != &signer.config)
    {
        return Err(unavailable("commit cache signer mismatch"));
    }
    if let Some(parent) = &state.parent {
        if current.commit().target.height < parent.height {
            return Err(unavailable("state rollback cannot clear approval pins"));
        }
        if current.commit().target != *parent || current.sequence() != state.sequence {
            *state = RegistryState::default();
        }
    }
    Ok(())
}
