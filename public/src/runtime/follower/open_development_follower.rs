// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    DevelopmentFollowerConfig,
    build_segmented_follower_configuration::build_segmented_follower_configuration,
    checkpoints::{
        bootstrap_development_checkpoint, build_checkpoint_recovery_configuration,
        open_development_checkpoint_directories,
    },
    validate_development_follower::validate_development_follower,
};
use crate::development::{read_development_spec, resolve_data_directory};
use crate::sync::applied::{
    AppliedOwner, AppliedReader, applied_commit, capture_applied_state,
    checkpoints::open_segmented_applied_state_service_with_checkpoints,
    finish_applied_state_service,
};
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
    let _existing_directories = open_development_checkpoint_directories(&path, false, None)?;
    let recovery = build_checkpoint_recovery_configuration(&path);
    let application = build_segmented_follower_configuration(path.clone(), identity)?;
    let (mut owner, reader) =
        open_segmented_applied_state_service_with_checkpoints(application, recovery, &genesis)
            .map_err(|error| anyhow::anyhow!("follower storage/bootstrap refused: {error:?}"))?;
    let requested = config.checkpoint_height;
    let current = capture_applied_state(&reader)
        .map_err(|error| anyhow::anyhow!("follower initial capture: {error:?}"));
    let bootstrap = match (current, requested) {
        (Ok(current), Some(height)) if height > applied_commit(&current).target.height => {
            bootstrap_development_checkpoint(
                &mut owner,
                &path,
                &build_checkpoint_recovery_configuration(&path),
                &genesis,
                height,
                config.validator_address,
            )
        }
        (Ok(_), _) => Ok(()),
        (Err(error), _) => Err(error),
    };
    if let Err(error) = bootstrap {
        let _shutdown = finish_applied_state_service(owner);
        return Err(error.context("follower checkpoint bootstrap refused; storage worker joined"));
    }
    Ok((owner, reader))
}
