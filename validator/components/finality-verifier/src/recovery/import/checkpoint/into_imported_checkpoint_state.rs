// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::ImportedState;
use super::AuthenticatedCheckpoint;
use std::sync::Arc;

/// Certified-import authority only. No independent replay conversion exists.
pub fn into_imported_checkpoint_state(checkpoint: AuthenticatedCheckpoint) -> Arc<ImportedState> {
    Arc::new(ImportedState {
        commit: checkpoint.commit,
        finality: checkpoint.finality,
        policy: checkpoint.policy,
        lookahead: Some(checkpoint.lookahead),
        lookahead_header: Some(checkpoint.lookahead_header),
        anchor: Some(checkpoint.anchor),
    })
}
