// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    capture_public_observation::capture_public_observation, types::PublicFollower,
    verify_public_observation::verify_public_observation,
};
use crate::support::{Cluster, collect_certified_history, replay_history};
use anyhow::{Context, Result, ensure};
use std::{collections::BTreeMap, time::Instant};

pub(in crate::cases) fn verify_public_current_state(
    cluster: &Cluster,
    follower: &mut PublicFollower,
    deadline: Instant,
) -> Result<()> {
    let observed = capture_public_observation(follower, cluster.authority_address, deadline)?;
    let height = u64::from_str_radix(
        observed.roots["height"]
            .as_str()
            .context("PUBLIC_FOLLOWER_ROOT_HEIGHT")?
            .strip_prefix("0x")
            .context("PUBLIC_FOLLOWER_ROOT_HEIGHT")?,
        16,
    )?;
    ensure!(
        height > 0 && height <= 256,
        "PUBLIC_FOLLOWER_ORACLE_PREFIX_BOUND"
    );
    let history =
        collect_certified_history(cluster, 0, i64::try_from(height)? + 1, &BTreeMap::new())?;
    let oracle = replay_history(cluster, &history)?;
    verify_public_observation(
        &observed,
        &oracle[height as usize],
        cluster.authority_address,
    )
}
