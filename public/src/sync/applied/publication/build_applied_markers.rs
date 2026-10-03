// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedError;
use crate::sync::applied::state::{AppliedState, applied_state_anchor, applied_state_commit};
use eve_node_policy::{
    AppliedHeight, AuthenticatedStateHeight, CheckpointHeight, DurableRecoveryHeight,
    FinalizedHeight, PublicWatermarks, StateAuthentication, validate_watermarks,
};

pub(in crate::sync::applied) fn build_applied_markers(
    state: &AppliedState,
    durable: u64,
) -> Result<PublicWatermarks, AppliedError> {
    let height = applied_state_commit(state).target.height;
    let (finalized, authentication) = match applied_state_anchor(state) {
        Some(anchor) => {
            let certified = u64::try_from(anchor.consensus_height())
                .map_err(|_| AppliedError::InvalidDurablePrefix)?;
            if anchor.execution_height() != height || height.checked_add(1) != Some(certified) {
                return Err(AppliedError::InvalidDurablePrefix);
            }
            (
                certified,
                StateAuthentication::NextCertifiedHeader {
                    header_height: certified,
                },
            )
        }
        None if height == 0 => (0, StateAuthentication::VerifiedReplay),
        None => return Err(AppliedError::InvalidDurablePrefix),
    };
    let markers = PublicWatermarks {
        finalized: FinalizedHeight(finalized),
        applied: AppliedHeight(height),
        durable_recovery: DurableRecoveryHeight(durable),
        checkpoint: CheckpointHeight(0),
        authenticated_state: AuthenticatedStateHeight(height),
        authentication,
        authenticated_snapshot_height: 0,
        oldest_retained_height: 0,
    };
    validate_watermarks(markers).map_err(|_| AppliedError::InvalidDurablePrefix)?;
    Ok(markers)
}
