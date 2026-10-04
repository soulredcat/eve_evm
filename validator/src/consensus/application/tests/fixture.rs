// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{ApplicationConfig, ConsensusApplication, open_application};
use crate::{
    consensus::signing::{SignerConfig, open_durable_signer},
    development::genesis::{
        build_development_consensus_params, build_development_validator_updates,
    },
};

use ed25519_dalek::SigningKey;
use eve_consensus_comet::consensus::{
    authentication::ConsensusAuthenticationRequirement, certificates::validator_address,
};
use eve_state::{DevelopmentGenesis, StateCommit, development_state_budget};
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
    path::Path,
    sync::{Arc, Mutex},
};

pub(in crate::consensus::application) struct TestApplication {
    pub root: tempfile::TempDir,
    pub genesis: Arc<StateCommit>,
    pub spec: DevelopmentGenesis,
    pub application: ConsensusApplication,
}

pub(in crate::consensus::application) fn test_application(revert: bool) -> TestApplication {
    let allowed = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("local-tests/b3-preparation/application-unit");
    std::fs::create_dir_all(&allowed).unwrap();
    let root = tempfile::Builder::new()
        .prefix("application-")
        .tempdir_in(allowed)
        .unwrap();
    let (genesis, spec) = super::genesis::application_test_genesis(revert);
    let application = reopen_application(root.path(), Arc::clone(&genesis), &spec);
    TestApplication {
        root,
        genesis,
        spec,
        application,
    }
}

pub(super) fn reopen_application(
    root: &Path,
    genesis: Arc<StateCommit>,
    spec: &DevelopmentGenesis,
) -> ConsensusApplication {
    let repository = open_state_repository(
        &root.join("state"),
        &genesis,
        development_state_storage_budget(),
    )
    .unwrap();
    let service = Arc::new(create_state_service(state_reader(&repository)));
    let key = SigningKey::from_bytes(&[1; 32]);
    let config = SignerConfig {
        genesis_hash: genesis.target.identity.genesis.0.0,
        chain_id: spec.network_name.clone(),
        expected_public_key: key.verifying_key().to_bytes(),
        authentication: ConsensusAuthenticationRequirement::ClassicalDev,
        key_epoch: 0,
    };
    let signer = Arc::new(Mutex::new(
        open_durable_signer(
            &root.join("signer"),
            config,
            key,
            Arc::clone(&service),
            development_opaque_record_budget(),
        )
        .unwrap(),
    ));
    let replay = open_opaque_record_repository(
        &root.join("replay"),
        OpaqueRecordIdentity {
            genesis_hash: genesis.target.identity.genesis.0.0,
            owner: [1; 32],
            domain: Sha256::digest(b"EVE_VALIDATOR_CONSENSUS_REPLAY_V1").into(),
        },
        development_opaque_record_budget(),
    )
    .unwrap();
    let owners: BTreeMap<_, _> = spec
        .validators
        .iter()
        .map(|validator| {
            (
                validator_address(&validator.classical_public_key),
                validator.owner,
            )
        })
        .collect();
    open_application(
        repository,
        service,
        replay,
        signer,
        ApplicationConfig {
            acceptance_fixture: None,
            genesis,
            initial_validators: build_development_validator_updates(spec).unwrap(),
            consensus_params: build_development_consensus_params(1),
            initial_time: prost_types::Timestamp {
                seconds: i64::try_from(spec.initial_timestamp).unwrap(),
                nanos: 0,
            },
            expected_app_state: serde_json::json!({"public_spec": "application-fixture"}),
            proposer_owners: owners,
            logical_budget: development_state_budget(),
            delta_serving_budget: super::super::development_delta_serving_budget(),
            snapshot_serving_budget: super::super::development_snapshot_serving_budget(),
            reserved_clone_bytes: 64 * 1_048_576,
            maximum_cached_candidates: 2,
            maximum_cached_bytes: 128 * 1_048_576,
        },
    )
    .unwrap()
}

/// Public existing independent signed vector, not a runtime key or producer default.
pub(super) fn transaction() -> Vec<u8> {
    hex::decode("f866808477359400830186a0940000000000000000000000000000000000000042808082f4f5a02c1d1a7db8b28ed638c4c0a70b8789ae4fc8d41fbf881cca9ccb0a97f8f487aba00a5013ea38ccecec4dab4c2c82674109c7a303ad69772b56ef82a47eef05d4c6").unwrap()
}
