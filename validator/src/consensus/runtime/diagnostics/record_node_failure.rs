// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::types::RunningNode;
use anyhow::Result;

pub(in crate::consensus::runtime) fn record_node_failure(
    node: &RunningNode,
    error: &anyhow::Error,
) -> Result<()> {
    let report = super::snapshot_node_failure::snapshot_node_failure(node, error);
    super::write_node_failure::write_node_failure(&node.assembly.config.data, &report)
}
