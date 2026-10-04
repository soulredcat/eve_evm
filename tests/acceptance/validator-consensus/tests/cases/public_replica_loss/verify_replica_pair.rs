// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    verify_replica_state::verify_replica_state, verify_replica_status::verify_replica_status,
};
use crate::cases::public_follower::{PublicFollower, wait_public_durable};
use crate::support::Cluster;
use alloy_primitives::Address;
use anyhow::Result;
use std::time::{Duration, Instant};

pub(super) fn verify_replica_pair(
    cluster: &Cluster,
    replicas: &mut [PublicFollower; 2],
    recipient: Address,
    minimum_durable: u64,
    expected_nonce: u64,
) -> Result<()> {
    let deadline = Instant::now() + Duration::from_secs(90);
    for replica in replicas.iter_mut() {
        let status = wait_public_durable(replica, minimum_durable, deadline)?;
        verify_replica_status(&status, minimum_durable)?;
    }
    for replica in replicas {
        verify_replica_state(
            cluster,
            replica,
            recipient,
            minimum_durable,
            expected_nonce,
            Instant::now() + Duration::from_secs(30),
        )?;
    }
    Ok(())
}
