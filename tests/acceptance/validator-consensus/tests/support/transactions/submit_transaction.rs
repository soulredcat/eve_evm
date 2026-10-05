// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Cluster;
use alloy_primitives::Bytes;
use anyhow::Result;
use std::time::{Duration, Instant};

/// Submit once through native admission and observe the exact committed execution.
///
/// A commit-waiting RPC holds one request open across proposal rounds and can
/// outlast the unchanged per-RPC limit on a loaded host. Admission and block
/// observation keep every request short under one 90-second progress budget.
pub(crate) fn submit_transaction(cluster: &Cluster, node: usize, bytes: &Bytes) -> Result<i64> {
    super::submit_transaction_until::submit_transaction_until(
        cluster,
        node,
        bytes,
        Instant::now() + Duration::from_secs(90),
    )
}
