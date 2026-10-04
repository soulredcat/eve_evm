// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{capture_durable_prefix::capture_durable_prefix, types::DurablePrefix};
use anyhow::{Result, ensure};
use serde_json::Value;

pub(super) fn verify_recovered_durability(
    status: &Value,
    previous: DurablePrefix,
    missing_tail_height: u64,
) -> Result<()> {
    let current = capture_durable_prefix(status, missing_tail_height)?;
    ensure!(
        current.height > previous.height
            && current.marker_sequence > previous.marker_sequence
            && current.acknowledged_physical_sequence >= previous.acknowledged_physical_sequence,
        "PUBLIC_TAIL_RECOVERY_DID_NOT_EXTEND_THE_RETAINED_PHYSICAL_PREFIX"
    );
    Ok(())
}
