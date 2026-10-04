// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointResponsePreflight, CheckpointResponseStats};
pub fn checkpoint_response_stats(
    preflight: &CheckpointResponsePreflight<'_>,
) -> CheckpointResponseStats {
    preflight.stats
}
