// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ApprovalRegistry, types::ApprovalRegistryStatus};
use crate::consensus::approval::ApprovalError;

pub(in crate::consensus) fn approval_registry_status(
    registry: &ApprovalRegistry,
) -> Result<ApprovalRegistryStatus, ApprovalError> {
    let state = registry
        .state
        .lock()
        .map_err(|_| ApprovalError::Unavailable {
            reason: "approval registry poisoned",
            cause: None,
        })?;
    Ok(ApprovalRegistryStatus {
        full: state.full.len(),
        raw: state.raw.len(),
        retained_encoded_bytes: state.raw.values().map(|value| value.bytes).sum(),
        current: state.current,
        prevote_pin: state.prevote_pin,
        precommit_pin: state.precommit_pin,
    })
}
