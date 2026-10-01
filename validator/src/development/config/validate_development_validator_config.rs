// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::DevelopmentValidatorConfig;
use anyhow::{Result, ensure};

pub(crate) fn validate_development_validator_config(
    config: &DevelopmentValidatorConfig,
) -> Result<()> {
    ensure!(
        cfg!(target_os = "linux"),
        "development validator requires the Linux reference platform"
    );
    ensure!(
        config.acknowledge_unsafe_development,
        "explicit unsafe development acknowledgement required"
    );
    ensure!(
        config.rpc_address.ip().is_loopback() && config.p2p_address.ip().is_loopback(),
        "development validator listeners must bind loopback"
    );
    ensure!(
        config
            .advertised_p2p_address
            .is_none_or(|address| address.ip().is_loopback() && address.port() != 0),
        "development advertised peer address must use loopback and a nonzero port"
    );
    ensure!(
        config.acceptance_fixture.is_none() || cfg!(feature = "development-acceptance"),
        "acceptance fixture requires the explicit disposable test build feature"
    );
    ensure!(
        config.comet_sha256.len() == 64
            && config
                .comet_sha256
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit()),
        "invalid expected engine digest"
    );
    ensure!(
        config.persistent_peers.len() <= 4096 && !config.persistent_peers.contains(['\n', '\r']),
        "invalid bounded persistent peer input"
    );
    ensure!(
        !config.data.as_os_str().is_empty(),
        "development data namespace required"
    );
    Ok(())
}
