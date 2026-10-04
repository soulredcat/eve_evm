// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::DevelopmentFollowerConfig;
use anyhow::{Result, ensure};

pub(super) fn validate_development_follower(config: &DevelopmentFollowerConfig) -> Result<()> {
    ensure!(
        config.acknowledge_unsafe_development,
        "CLASSICAL_DEV follower requires explicit unsafe development acknowledgment"
    );
    ensure!(
        config.validator_address.ip().is_loopback()
            && config.validator_address.port() != 0
            && config.http_address.ip().is_loopback()
            && config.ws_address.ip().is_loopback(),
        "development follower endpoints must use loopback"
    );
    ensure!(
        (50..=30_000).contains(&config.poll_interval_ms),
        "invalid follower polling interval"
    );
    ensure!(
        !config.node_name.is_empty()
            && config.node_name.len() <= 64
            && config
                .node_name
                .bytes()
                .all(|byte| byte.is_ascii_alphanumeric() || b"_-".contains(&byte)),
        "invalid follower namespace name"
    );
    ensure!(
        config
            .checkpoint_height
            .is_none_or(|height| (1..=super::checkpoints::CHECKPOINT_MAX_HEIGHT).contains(&height)),
        "checkpoint height must be between 1 and 10000"
    );
    Ok(())
}
