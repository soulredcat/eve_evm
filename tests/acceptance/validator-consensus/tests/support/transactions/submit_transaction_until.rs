// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Cluster;
use alloy_primitives::Bytes;
use anyhow::Result;
use std::time::Instant;

pub(crate) fn submit_transaction_until(
    cluster: &Cluster,
    node: usize,
    bytes: &Bytes,
    deadline: Instant,
) -> Result<i64> {
    super::submit_transaction_to_until::submit_transaction_to_until(
        cluster.nodes[node].rpc,
        bytes,
        deadline,
    )
}
