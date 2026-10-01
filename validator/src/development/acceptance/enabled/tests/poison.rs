// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{fixture, rebind};
use crate::development::acceptance::enabled::{
    load_acceptance_fixture, poison_development_proposal,
};
use alloy_primitives::{Address, B256, Bytes};
use eve_consensus_comet::{
    consensus::certificates::validator_address, wire::tendermint::abci::RequestPrepareProposal,
};
use eve_evm::{ExecutionBlockInput, execute_state_block};
use eve_state::{development_state_budget, initialize_development_state};
use std::sync::atomic::{AtomicBool, Ordering};

#[test]
fn poison_requires_exact_initial_proposer_height_and_is_consumed_only_once() {
    let mut material = fixture();
    let key = material.genesis.validators[0].classical_public_key;
    material.document["poison"] = serde_json::json!({"height":2,"public_key":hex::encode(key)});
    rebind(&mut material);
    let capability = material.load();
    let used = AtomicBool::new(false);
    let mut request = RequestPrepareProposal {
        height: 1,
        proposer_address: validator_address(&key).to_vec(),
        ..Default::default()
    };
    assert!(poison_development_proposal(Some(&capability), &request, &used).is_none());
    assert!(!used.load(Ordering::Acquire));
    request.height = 2;
    request.proposer_address = vec![9; 20];
    assert!(poison_development_proposal(Some(&capability), &request, &used).is_none());
    assert!(!used.load(Ordering::Acquire));
    request.proposer_address = validator_address(&key).to_vec();
    assert!(poison_development_proposal(None, &request, &used).is_none());
    assert_eq!(
        poison_development_proposal(Some(&capability), &request, &used),
        Some(vec![vec![0xff]])
    );
    assert!(used.load(Ordering::Acquire));
    assert!(poison_development_proposal(Some(&capability), &request, &used).is_none());
}

#[test]
fn selected_poison_cannot_execute_or_produce_the_state_required_for_nonnil_approval() {
    let mut material = fixture();
    let key = material.genesis.validators[0].classical_public_key;
    material.document["poison"] = serde_json::json!({"height":1,"public_key":hex::encode(key)});
    rebind(&mut material);
    let capability = material.load();
    let request = RequestPrepareProposal {
        height: 1,
        proposer_address: validator_address(&key).to_vec(),
        ..Default::default()
    };
    let raw =
        poison_development_proposal(Some(&capability), &request, &AtomicBool::new(false)).unwrap();
    let transactions: Vec<Bytes> = raw.into_iter().map(Bytes::from).collect();
    let budget = development_state_budget();
    let genesis = initialize_development_state(&material.genesis, &budget).unwrap();
    let before = genesis.target.clone();
    assert!(
        execute_state_block(
            &genesis,
            &ExecutionBlockInput {
                timestamp: 2,
                proposer: Address::repeat_byte(4),
                previous_consensus_hash: B256::ZERO
            },
            &transactions,
            &budget,
            64 * 1_048_576
        )
        .is_err()
    );
    assert_eq!(genesis.target, before);
    assert!(genesis.target.application.is_none());
}

#[test]
fn poison_rejects_unbounded_heights_and_future_or_unknown_proposer_keys() {
    for mutation in 0..4 {
        let mut material = fixture();
        let key = material.genesis.validators[0].classical_public_key;
        material.document["poison"] = serde_json::json!({"height":1,"public_key":hex::encode(key)});
        match mutation {
            0 => material.document["poison"]["height"] = 0.into(),
            1 => material.document["poison"]["height"] = 1001.into(),
            2 => {
                material.document["poison"]["public_key"] =
                    material.document["future_validators"][0]["public_key"].clone()
            }
            _ => material.document["poison"]["public_key"] = hex::encode([2; 32]).into(),
        }
        rebind(&mut material);
        assert!(load_acceptance_fixture(Some(&material.path), &material.genesis).is_err());
    }
}
