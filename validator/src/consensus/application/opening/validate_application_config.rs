// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::matches_genesis_validator_record::matches_genesis_validator_record;
use crate::consensus::application::ApplicationConfig;
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::{
    consensus::certificates::{
        ClassicalValidator, canonicalize_validator_set, hash_validator_set, validator_address,
    },
    wire::tendermint::crypto::public_key::Sum,
};
use eve_state::{SecurityProfile, validate_state_commit};

pub(super) fn validate_application_config(config: &ApplicationConfig) -> Result<()> {
    validate_state_commit(&config.genesis, &config.logical_budget)
        .map_err(|_| anyhow::anyhow!("invalid canonical application genesis"))?;
    ensure!(
        config.genesis.target.height == 0
            && config.genesis.target.application.is_none()
            && config.genesis.target.identity.security_profile == SecurityProfile::ClassicalDev
            && config.genesis.target.identity.protocol_version == 1,
        "unsupported development application identity"
    );
    ensure!(
        config.initial_time.seconds >= 0
            && u64::try_from(config.initial_time.seconds).ok()
                == Some(config.genesis.target.timestamp)
            && config.initial_time.nanos == 0,
        "application genesis timestamp mismatch"
    );
    ensure!(
        config.initial_validators.len() == 4
            && config.maximum_cached_candidates == 2
            && config.maximum_cached_bytes > 0
            && config.reserved_clone_bytes > 0,
        "invalid application development resource configuration"
    );
    let block = config
        .consensus_params
        .block
        .as_ref()
        .context("native block parameters missing")?;
    ensure!(
        block.max_bytes == 4_194_304 && block.max_gas == 30_000_000,
        "native development block parameters mismatch"
    );
    let key_types = &config
        .consensus_params
        .validator
        .as_ref()
        .context("native validator parameters missing")?
        .pub_key_types;
    ensure!(
        key_types == &["ed25519"],
        "unsupported native validator key types"
    );
    let mut validators = Vec::new();
    for update in &config.initial_validators {
        let key = match update.pub_key.as_ref().and_then(|key| key.sum.as_ref()) {
            Some(Sum::Ed25519(key)) => key.as_slice().try_into().context("native key width")?,
            _ => anyhow::bail!("unsupported native genesis key"),
        };
        let owner = config
            .proposer_owners
            .get(&validator_address(&key))
            .context("native proposer-owner mapping missing")?;
        ensure!(
            config
                .genesis
                .state
                .system
                .values()
                .any(|record| matches_genesis_validator_record(record, owner, &key, update.power)),
            "native validator differs from canonical genesis registry"
        );
        validators.push(ClassicalValidator {
            public_key: key,
            voting_power: update.power,
        });
    }
    hash_validator_set(
        &canonicalize_validator_set(&validators)
            .map_err(|_| anyhow::anyhow!("invalid native genesis set"))?,
    )
    .map_err(|_| anyhow::anyhow!("invalid native genesis set hash"))?;
    crate::development::acceptance::validate_acceptance_proposer_owners(
        config.acceptance_fixture.as_deref(),
        &validators,
        &config.proposer_owners,
    )?;
    Ok(())
}
