// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{RecoveryChain, build_recovery_chain::build_recovery_chain};

/// Exact preserved nonce-zero followed by two empty heights.
pub fn recovery_chain() -> RecoveryChain {
    build_recovery_chain(false)
}
