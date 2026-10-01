// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::load_node_public_spec::load_node_public_spec;
use crate::{
    consensus::{
        application::{ApplicationConfig, application_approval_registry, open_application},
        runtime::{
            namespace::{open_node_namespace, secure_repository_namespace},
            types::{DevelopmentValidatorInitialization, NodeAssembly, NodeIdentity},
        },
        signing::{SignerConfig, open_durable_signer},
    },
    development::{
        acceptance::{extend_acceptance_proposer_owners, load_acceptance_fixture},
        config::{
            DevelopmentValidatorConfig, load_development_validator_key,
            validate_development_validator_config, validate_development_validator_keys,
        },
        engine::{engine_node_id, initialize_engine_home},
        genesis::{
            build_development_consensus_params, build_development_validator_updates,
            build_native_genesis_document,
        },
    },
};
use anyhow::Result;
use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement, certificates::validator_address,
};
use eve_state::{development_state_budget, initialize_development_state};
use eve_storage::{
    records::{
        OpaqueRecordIdentity, development_opaque_record_budget, open_opaque_record_repository,
    },
    state::{
        create_state_service, development_state_storage_budget, open_state_repository, state_reader,
    },
};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

pub(in crate::consensus::runtime) fn assemble_development_node(
    config: DevelopmentValidatorConfig,
) -> Result<NodeAssembly> {
    validate_development_validator_config(&config)?;
    let digest = hex::decode(&config.comet_sha256)?
        .try_into()
        .map_err(|_| anyhow::anyhow!("invalid expected engine digest"))?;
    let (spec, app_state) = load_node_public_spec(&config.genesis)?;
    let key = load_development_validator_key(&config.signing_seed)?;
    let acceptance_fixture = load_acceptance_fixture(config.acceptance_fixture.as_deref(), &spec)?;
    validate_development_validator_keys(&spec, &key, acceptance_fixture.as_deref())?;
    let public_key = key.verifying_key().to_bytes();
    let budget = development_state_budget();
    let genesis = Arc::new(
        initialize_development_state(&spec, &budget)
            .map_err(|_| anyhow::anyhow!("invalid canonical development genesis state"))?,
    );
    let genesis_hash = genesis.target.identity.genesis.0.0;
    let lease = open_node_namespace(
        &config.data,
        &NodeIdentity {
            schema: 1,
            genesis_hash: hex::encode(genesis_hash),
            public_key: hex::encode(public_key),
            engine_sha256: hex::encode(digest),
        },
    )?;
    let native_genesis = build_native_genesis_document(&spec, &genesis, app_state.clone())?;
    let home = config.data.join("engine");
    initialize_engine_home(&config, &home, &native_genesis, digest)?;
    let node_id = engine_node_id(&config, &home, digest)?;
    let repository = open_state_repository(
        &config.data.join("state"),
        &genesis,
        development_state_storage_budget(),
    )?;
    secure_repository_namespace(&config.data.join("state"))?;
    let service = Arc::new(create_state_service(state_reader(&repository)));
    let signer = Arc::new(Mutex::new(open_durable_signer(
        &config.data.join("signer"),
        SignerConfig {
            genesis_hash,
            chain_id: spec.network_name.clone(),
            expected_public_key: public_key,
            authentication: ConsensusAuthenticationRequirement::ClassicalDev,
            key_epoch: 0,
        },
        key,
        Arc::clone(&service),
        development_opaque_record_budget(),
    )?));
    secure_repository_namespace(&config.data.join("signer"))?;
    let replay = open_opaque_record_repository(
        &config.data.join("replay"),
        OpaqueRecordIdentity {
            genesis_hash,
            owner: public_key,
            domain: Sha256::digest(b"EVE_VALIDATOR_CONSENSUS_REPLAY_V1").into(),
        },
        development_opaque_record_budget(),
    )?;
    secure_repository_namespace(&config.data.join("replay"))?;
    let mut owners: BTreeMap<_, _> = spec
        .validators
        .iter()
        .map(|validator| {
            (
                validator_address(&validator.classical_public_key),
                validator.owner,
            )
        })
        .collect();
    extend_acceptance_proposer_owners(acceptance_fixture.as_deref(), &mut owners)?;
    let application = open_application(
        repository,
        service,
        replay,
        Arc::clone(&signer),
        ApplicationConfig {
            acceptance_fixture,
            genesis,
            initial_validators: build_development_validator_updates(&spec)?,
            consensus_params: build_development_consensus_params(spec.protocol_version),
            initial_time: prost_types::Timestamp {
                seconds: i64::try_from(spec.initial_timestamp)?,
                nanos: 0,
            },
            expected_app_state: app_state,
            proposer_owners: owners,
            logical_budget: budget,
            reserved_clone_bytes: 64 * 1_048_576,
            maximum_cached_candidates: 2,
            maximum_cached_bytes: 128 * 1_048_576,
        },
    )?;
    let registry = application_approval_registry(&application);
    let initialization = DevelopmentValidatorInitialization {
        node_id,
        chain_id: spec.network_name,
        genesis_hash: hex::encode(genesis_hash),
        rpc_address: config.rpc_address,
        p2p_address: config.p2p_address,
        zone_id: config.zone_id,
        security_profile: "CLASSICAL_DEV",
    };
    Ok(NodeAssembly {
        config,
        initialization,
        digest,
        application: Arc::new(Mutex::new(application)),
        signer,
        registry,
        lease,
    })
}
