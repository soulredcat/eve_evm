// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::chain;
use eve_finality_verifier::{FinalityError, authenticate_current_application_version};
use eve_protocol_config::records::{ApplicationCommitmentInput, hash_application_commitment};
use eve_state::{B256, SecurityProfile};

#[test]
fn real_execution_outcome_is_anchored_only_by_its_exact_certified_next_header() {
    let chain = chain::chain();
    let mut verifier = chain::verifier(&chain.genesis);
    chain::accept(&mut verifier, &chain.frames[0]);
    let version = &chain.commits[1].target;
    assert_eq!(
        authenticate_current_application_version(&verifier, version).err(),
        Some(FinalityError::WrongApplicationHeight)
    );
    chain::accept(&mut verifier, &chain.frames[1]);
    let anchor = authenticate_current_application_version(&verifier, version).unwrap();
    assert_eq!(anchor.identity(), &version.identity);
    assert_eq!(anchor.execution_height(), 1);
    assert_eq!(anchor.evm_root(), version.evm_root);
    assert_eq!(anchor.system_root(), version.system_root);
    assert_eq!(anchor.execution_hash(), version.execution_hash);
    assert_eq!(anchor.application(), version.application.unwrap());
    assert_eq!(anchor.consensus_height(), 2);
    assert_eq!(anchor.consensus_block_id(), &chain.frames[1].id);
    chain::accept(&mut verifier, &chain.frames[2]);
    assert_eq!(
        authenticate_current_application_version(&verifier, version).err(),
        Some(FinalityError::WrongApplicationHeight)
    );
    assert!(authenticate_current_application_version(&verifier, &chain.commits[2].target).is_ok());
}

#[test]
fn network_genesis_profile_configuration_epoch_and_protocol_identity_cannot_be_substituted() {
    let chain = chain::chain();
    let mut verifier = chain::verifier(&chain.genesis);
    chain::accept(&mut verifier, &chain.frames[0]);
    chain::accept(&mut verifier, &chain.frames[1]);
    let valid = &chain.commits[1].target;
    for field in 0..7 {
        let mut version = valid.clone();
        match field {
            0 => version.identity.network_name.push_str("-foreign"),
            1 => version.identity.evm_chain_id += 1,
            2 => version.identity.genesis.0 = B256::repeat_byte(0x90),
            3 => version.identity.security_profile = SecurityProfile::HybridExperimental,
            4 => version.identity.configuration_digest = B256::repeat_byte(0x91),
            5 => version.identity.key_epoch += 1,
            6 => version.identity.protocol_version += 1,
            _ => unreachable!(),
        }
        assert_eq!(
            authenticate_current_application_version(&verifier, &version).err(),
            Some(FinalityError::WrongApplicationIdentity)
        );
    }
    assert!(authenticate_current_application_version(&verifier, valid).is_ok());
}

#[test]
fn changed_roots_execution_hash_application_and_height_never_share_the_valid_anchor() {
    let chain = chain::chain();
    let mut verifier = chain::verifier(&chain.genesis);
    chain::accept(&mut verifier, &chain.frames[0]);
    chain::accept(&mut verifier, &chain.frames[1]);
    let valid = &chain.commits[1].target;
    for field in 0..3 {
        let mut changed = valid.clone();
        match field {
            0 => changed.evm_root.0 = B256::repeat_byte(0x81),
            1 => changed.system_root.0 = B256::repeat_byte(0x82),
            2 => changed.execution_hash.0 = B256::repeat_byte(0x83),
            _ => unreachable!(),
        }
        assert_eq!(
            authenticate_current_application_version(&verifier, &changed).err(),
            Some(FinalityError::WrongApplicationCommitment)
        );
        changed.application = Some(
            hash_application_commitment(ApplicationCommitmentInput {
                genesis: changed.identity.genesis,
                protocol_version: changed.identity.protocol_version,
                execution_height: changed.height,
                evm_root: changed.evm_root,
                system_root: changed.system_root,
                execution_hash: changed.execution_hash,
            })
            .unwrap(),
        );
        assert!(authenticate_current_application_version(&verifier, &changed).is_err());
    }
    let mut missing = valid.clone();
    missing.application = None;
    assert_eq!(
        authenticate_current_application_version(&verifier, &missing).err(),
        Some(FinalityError::WrongApplicationCommitment)
    );
    let mut changed_application = valid.clone();
    changed_application.application.as_mut().unwrap().0.0 = [0xa1; 32];
    assert_eq!(
        authenticate_current_application_version(&verifier, &changed_application).err(),
        Some(FinalityError::WrongApplicationCommitment)
    );
    let mut wrong_height = valid.clone();
    wrong_height.height += 1;
    assert_eq!(
        authenticate_current_application_version(&verifier, &wrong_height).err(),
        Some(FinalityError::WrongApplicationHeight)
    );
    assert!(authenticate_current_application_version(&verifier, valid).is_ok());
}

#[test]
fn auxiliary_timestamp_and_content_digest_are_not_claimed_as_application_authenticated() {
    let chain = chain::chain();
    let mut verifier = chain::verifier(&chain.genesis);
    chain::accept(&mut verifier, &chain.frames[0]);
    chain::accept(&mut verifier, &chain.frames[1]);
    let valid = &chain.commits[1].target;
    let mut auxiliary = valid.clone();
    auxiliary.timestamp += 999;
    auxiliary.content_digest = B256::repeat_byte(0xfe);
    let original = authenticate_current_application_version(&verifier, valid).unwrap();
    let changed = authenticate_current_application_version(&verifier, &auxiliary).unwrap();
    assert_eq!(original.identity(), changed.identity());
    assert_eq!(original.execution_height(), changed.execution_height());
    assert_eq!(original.evm_root(), changed.evm_root());
    assert_eq!(original.system_root(), changed.system_root());
    assert_eq!(original.execution_hash(), changed.execution_hash());
    assert_eq!(original.application(), changed.application());
    assert_eq!(original.consensus_height(), changed.consensus_height());
    assert_eq!(original.consensus_block_id(), changed.consensus_block_id());
}
