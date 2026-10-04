// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AuthenticatedCheckpoint, CheckpointError, CheckpointSession};
use crate::{authenticate_current_application_version, recovery::RecoveryError};

pub fn finish_authenticated_checkpoint(
    session: CheckpointSession,
) -> Result<AuthenticatedCheckpoint, CheckpointError> {
    let lookahead = session.closing_data.ok_or(CheckpointError::Incomplete)?;
    let lookahead_header = session.closing_header.ok_or(CheckpointError::Incomplete)?;
    let anchor =
        authenticate_current_application_version(&session.finality, &session.target.target)
            .map_err(|error| CheckpointError::Recovery(RecoveryError::Finality(error)))?;
    Ok(AuthenticatedCheckpoint {
        commit: session.target,
        finality: session.finality,
        policy: session.policy,
        lookahead,
        lookahead_header,
        anchor,
    })
}
