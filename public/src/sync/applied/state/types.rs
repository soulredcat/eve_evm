// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_finality_verifier::{DevelopmentRecoveryState, ImportedState};
use std::sync::Arc;

/// Private capabilities remain distinct. No imported-to-replayed conversion exists.
pub(in crate::sync::applied) enum AppliedState {
    EmptyReplay(Arc<DevelopmentRecoveryState>),
    AuthenticatedImport(Arc<ImportedState>),
}
