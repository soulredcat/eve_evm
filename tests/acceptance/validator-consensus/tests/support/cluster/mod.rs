// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod create_cluster;
mod node_height;
mod start_cluster;
mod stop_cluster;
mod types;
mod verify_height_deadline;
mod wait_for_height;
mod wait_for_height_until;
mod write_height_deadline_diagnostics;

pub(crate) use node_height::node_height;
use std::sync::{Mutex, MutexGuard};
pub(crate) use types::{Cluster, ClusterOptions, Node};
pub(crate) use wait_for_height::wait_for_height;
pub(crate) use wait_for_height_until::wait_for_height_until;
static CLUSTER_LEASE: Mutex<()> = Mutex::new(());
pub(crate) fn cluster_lease() -> MutexGuard<'static, ()> {
    CLUSTER_LEASE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}
