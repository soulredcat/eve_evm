// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod create_cluster;
mod start_cluster;
mod stop_cluster;
mod types;
mod wait_for_height;

use std::sync::{Mutex, MutexGuard};
pub(crate) use types::{Cluster, ClusterOptions, Node};
pub(crate) use wait_for_height::{node_height, wait_for_height};
static CLUSTER_LEASE: Mutex<()> = Mutex::new(());
pub(crate) fn cluster_lease() -> MutexGuard<'static, ()> {
    CLUSTER_LEASE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
