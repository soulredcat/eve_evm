// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::runtime::types::RunningNode;
use anyhow::Result;
use std::sync::atomic::Ordering;

pub(in crate::consensus::runtime) fn report_channel_completion(
    node: &RunningNode,
    outcome: Result<()>,
) {
    if node.stop.load(Ordering::Acquire) {
        return;
    }
    let error = outcome
        .err()
        .unwrap_or_else(|| anyhow::anyhow!("native channel worker exited before shutdown"));
    let _ =
        super::capture_first_channel_failure::capture_first_channel_failure(&node.failure, error);
    node.stop.store(true, Ordering::Release);
}
