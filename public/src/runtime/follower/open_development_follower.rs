// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    DevelopmentFollowerConfig,
    build_segmented_follower_configuration::build_segmented_follower_configuration,
    validate_development_follower::validate_development_follower,
};
use crate::development::{read_development_spec, resolve_data_directory};
use crate::sync::applied::{AppliedOwner, AppliedReader, open_segmented_applied_state_service};
use anyhow::{Result, ensure};
use eve_protocol_config::{
    genesis::hash_development_genesis,
    network::{LaunchMode, SecurityProfile},
};
use eve_storage::records::OpaqueRecordIdentity;
use sha2::{Digest, Sha256};

pub(super) fn open_development_follower(
    config: &DevelopmentFollowerConfig,
) -> Result<(AppliedOwner, AppliedReader), anyhow::Error> {
    validate_development_follower(config)?;
    let genesis = read_development_spec(&config.genesis)?;
    ensure!(
        genesis.profile == SecurityProfile::ClassicalDev,
        "follower supports only explicit CLASSICAL_DEV"
    );
    let genesis_hash = hash_development_genesis(LaunchMode::Development, &genesis)
        .map_err(|error| anyhow::anyhow!("invalid follower genesis: {error:?}"))?;
    let mut owner = Sha256::new();
    owner.update(b"EVE_PUBLIC_FOLLOWER_OWNER_V1");
    owner.update(genesis_hash.0);
    owner.update(config.node_name.as_bytes());
    let identity = OpaqueRecordIdentity {
        genesis_hash: genesis_hash.0,
        owner: owner.finalize().into(),
        domain: Sha256::digest(b"EVE_PUBLIC_FOLLOWER_RECOVERY_V1").into(),
    };
    let path = resolve_data_directory(&config.root, &config.data)?;
    let application = build_segmented_follower_configuration(path, identity)?;
    open_segmented_applied_state_service(application, &genesis)
        .map_err(|error| anyhow::anyhow!("follower storage/bootstrap refused: {error:?}"))
}
