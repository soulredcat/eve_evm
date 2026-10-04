// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{StateDeltaPayloadPreflight, StateDeltaPayloadStats};

pub fn state_delta_payload_stats(
    preflight: &StateDeltaPayloadPreflight<'_>,
) -> StateDeltaPayloadStats {
    preflight.stats
}
