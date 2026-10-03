// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use crate::recovery::{DevelopmentRecoveryState, VerifiedRecoveryTransition};

/// Consuming a verified transition establishes no queue admission or synced storage.
pub fn into_recovery_state(
    transition: VerifiedRecoveryTransition,
) -> Arc<DevelopmentRecoveryState> {
    transition.state
}
