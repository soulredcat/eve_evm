// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::{CompactRecoveryEnvelopeV1, DevelopmentRecoveryState, RecoveryError};

pub(in crate::recovery) fn validate_recovery_parent(
    parent: &DevelopmentRecoveryState,
    envelope: &CompactRecoveryEnvelopeV1,
) -> Result<(), RecoveryError> {
    let current = &parent.commit.target;
    if envelope.parent != *current {
        return Err(RecoveryError::WrongParent);
    }
    if envelope.expected.identity != current.identity {
        return Err(RecoveryError::WrongIdentity);
    }
    let height = current
        .height
        .checked_add(1)
        .ok_or(RecoveryError::WrongHeight)?;
    let next = height.checked_add(1).ok_or(RecoveryError::WrongHeight)?;
    if envelope.expected.height != height
        || i64::try_from(height).ok() != Some(envelope.finalized.header.height)
        || i64::try_from(next).ok() != Some(envelope.lookahead.frame.header.height)
    {
        return Err(RecoveryError::WrongHeight);
    }
    Ok(())
}
