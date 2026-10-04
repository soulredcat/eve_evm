// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{StateCommitDecodeStats, StateCommitPreflight};
pub fn state_commit_preflight_stats(
    preflight: &StateCommitPreflight<'_>,
) -> StateCommitDecodeStats {
    preflight.stats
}
