// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::cases::public_follower::{PublicFollower, read_public_startup, spawn_public_follower};
use anyhow::{Result, ensure};
use std::time::{Duration, Instant};

/// Both concurrent real followers must start from their configured empty data.
pub(super) fn start_replica_pair(replicas: &mut [PublicFollower; 2]) -> Result<()> {
    for replica in replicas.iter_mut() {
        ensure!(
            replica.child.is_none()
                && replica.checkpoint_height.is_none()
                && !replica.data.exists(),
            "PUBLIC_REPLICA_EMPTY_UNREQUESTED_CHECKPOINT_BOOTSTRAP"
        );
        spawn_public_follower(replica)?;
    }
    let deadline = Instant::now() + Duration::from_secs(90);
    for replica in replicas {
        let startup = read_public_startup(replica, deadline)?;
        ensure!(
            startup["height"] == 0 && startup["authenticated_finality"] == false,
            "PUBLIC_REPLICA_STARTUP_DID_NOT_USE_GENESIS"
        );
    }
    Ok(())
}
