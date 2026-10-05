// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::{
    CompactRecoveryEnvelopeV1, DevelopmentRecoveryState, RecoveryError,
    types::capability::PreparedRecoveryHistory,
};

use super::verify_authenticated_recovery_history::{
    RecoveryHistoryInput, RecoveryHistoryParent, verify_authenticated_recovery_history,
};

/// Stage H/H+1 on a clone; failures never advance the accepted parent's history.
pub(in crate::recovery) fn verify_recovery_history(
    parent: &DevelopmentRecoveryState,
    envelope: &CompactRecoveryEnvelopeV1,
) -> Result<PreparedRecoveryHistory, RecoveryError> {
    verify_authenticated_recovery_history(
        &RecoveryHistoryParent {
            finality: &parent.finality,
            height: parent.commit.target.height,
            lookahead: parent.lookahead.as_deref(),
            lookahead_header: parent.lookahead_header.as_ref(),
            policy: &parent.policy,
        },
        &RecoveryHistoryInput {
            finalized: &envelope.finalized,
            transactions: &envelope.execution.transactions,
            lookahead: &envelope.lookahead,
        },
    )
}
