// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::Cluster;
use anyhow::Result;
use std::time::{Duration, Instant};

pub(crate) fn wait_for_height(cluster: &mut Cluster, nodes: &[usize], height: i64) -> Result<()> {
    super::wait_for_height_until::wait_for_height_until(
        cluster,
        nodes,
        height,
        Instant::now() + Duration::from_secs(90),
    )
}
