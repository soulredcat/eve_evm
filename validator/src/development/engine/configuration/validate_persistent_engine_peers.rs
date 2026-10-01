// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{collections::BTreeSet, net::SocketAddr};

pub(in crate::development::engine) fn validate_persistent_engine_peers(peers: &str) -> Result<()> {
    ensure!(peers.len() <= 4096, "engine peer-list byte limit");
    if peers.is_empty() {
        return Ok(());
    }
    let mut identities = BTreeSet::new();
    let mut count = 0;
    for peer in peers.split(',') {
        count += 1;
        ensure!(count <= 16, "engine peer-count limit");
        let (identity, address) = peer
            .split_once('@')
            .ok_or_else(|| anyhow::anyhow!("invalid engine peer syntax"))?;
        ensure!(
            identity.len() == 40 && identity.bytes().all(|byte| byte.is_ascii_hexdigit()),
            "invalid full-width native node ID"
        );
        ensure!(
            identities.insert(identity.to_ascii_lowercase()),
            "duplicate engine node ID"
        );
        let address: SocketAddr = address.parse()?;
        ensure!(
            address.ip().is_loopback() && address.port() != 0,
            "development engine peers require loopback nonzero ports"
        );
    }
    Ok(())
}
