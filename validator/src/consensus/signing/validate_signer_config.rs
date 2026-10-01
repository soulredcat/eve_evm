// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::SignerConfig;
use anyhow::{Result, ensure};
use ed25519_dalek::SigningKey;
use eve_consensus_comet::consensus::authentication::require_supported_authentication;
use eve_state::SecurityProfile;
use eve_storage::state::{StateService, read_state_service};

pub(in crate::consensus) fn validate_signer_config(
    config: &SignerConfig,
    key: &SigningKey,
    service: &StateService,
) -> Result<()> {
    ensure!(
        require_supported_authentication(config.authentication).is_ok(),
        "unsupported activated signer profile"
    );
    ensure!(
        !config.chain_id.is_empty() && config.chain_id.len() <= 50,
        "invalid signer chain identity"
    );
    ensure!(config.genesis_hash != [0; 32], "empty signer genesis");
    ensure!(
        key.verifying_key().to_bytes() == config.expected_public_key,
        "signer key enrollment mismatch"
    );
    let view = read_state_service(service)?;
    let identity = &view.commit().target.identity;
    ensure!(
        identity.genesis.0.as_slice() == config.genesis_hash,
        "signer state genesis mismatch"
    );
    ensure!(
        identity.network_name == config.chain_id,
        "signer state chain mismatch"
    );
    ensure!(
        identity.security_profile == SecurityProfile::ClassicalDev,
        "unsupported state authentication profile"
    );
    ensure!(
        identity.key_epoch == config.key_epoch,
        "signer state key epoch mismatch"
    );
    Ok(())
}
