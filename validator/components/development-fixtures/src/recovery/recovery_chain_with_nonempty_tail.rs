// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{RecoveryChain, build_recovery_chain::build_recovery_chain};
pub fn recovery_chain_with_nonempty_tail() -> RecoveryChain {
    build_recovery_chain(true)
}
