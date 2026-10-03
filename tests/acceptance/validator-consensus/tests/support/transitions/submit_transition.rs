// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{Cluster, transactions::submit_transaction_until};
use alloy_primitives::Bytes;
use anyhow::{Context, Result};
use std::time::Instant;

/// Identify the fixture action without exposing its transaction or authority data.
pub(crate) fn submit_transition(
    cluster: &Cluster,
    bytes: &Bytes,
    action: u8,
    deadline: Instant,
) -> Result<i64> {
    let phase = match action {
        1 => "B3_TC07_SUBMIT_ROTATION",
        2 => "B3_TC07_SUBMIT_LEAVE",
        3 => "B3_TC07_SUBMIT_JAIL",
        _ => anyhow::bail!("B3_TC07_ACTION_INVALID"),
    };
    submit_transaction_until(cluster, 3, bytes, deadline).context(phase)
}
