// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::types::RunningNode;

pub(in crate::consensus::runtime) fn handle_channel_worker_drop(node: &RunningNode) {
    super::super::report_channel_completion::report_channel_completion(
        node,
        Err(anyhow::anyhow!(
            "native channel worker terminated unexpectedly"
        )),
    );
}
