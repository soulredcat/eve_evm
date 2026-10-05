// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::sync::Arc;

use crate::recovery_support::{self as support, CLONE_BYTES};
use eve_finality_verifier::{
    RecoveryError, initialize_development_recovery, prepare_development_recovery,
    recovery_state_commit,
};
use eve_state::{B256, SecurityProfile, development_state_budget};

#[test]
fn exact_parent_version_rejects_auxiliary_timestamp_and_content_digest_substitution() {
    let chain = support::recovery_chain();
    let parent =
        initialize_development_recovery(&chain.genesis, &development_state_budget()).unwrap();
    for field in 0..2 {
        let mut record = support::envelope(&chain, 1);
        if field == 0 {
            record.parent.timestamp += 1;
        } else {
            record.parent.content_digest = B256::repeat_byte(0xb1);
        }
        assert_eq!(
            prepare_development_recovery(
                &parent,
                Arc::new(record),
                &development_state_budget(),
                CLONE_BYTES,
            )
            .unwrap_err(),
            RecoveryError::WrongParent,
        );
    }
    assert_eq!(recovery_state_commit(&parent), &chain.commits[0]);
}

#[test]
fn expected_network_genesis_protocol_configuration_profile_and_epoch_are_immutable() {
    let chain = support::recovery_chain();
    let parent =
        initialize_development_recovery(&chain.genesis, &development_state_budget()).unwrap();
    for field in 0..7 {
        let mut record = support::envelope(&chain, 1);
        let identity = &mut record.expected.identity;
        match field {
            0 => identity.network_name.push_str("-foreign"),
            1 => identity.evm_chain_id += 1,
            2 => identity.genesis.0 = B256::repeat_byte(0x31),
            3 => identity.protocol_version += 1,
            4 => identity.configuration_digest = B256::repeat_byte(0x32),
            5 => identity.security_profile = SecurityProfile::HybridExperimental,
            6 => identity.key_epoch += 1,
            _ => unreachable!(),
        }
        assert!(matches!(
            prepare_development_recovery(
                &parent,
                Arc::new(record),
                &development_state_budget(),
                CLONE_BYTES,
            )
            .unwrap_err(),
            RecoveryError::WrongIdentity | RecoveryError::State(_),
        ));
    }
    assert_eq!(recovery_state_commit(&parent), &chain.commits[0]);
}

#[test]
fn missing_shifted_or_overflowing_h_h_plus_one_heights_cannot_advance_state() {
    let chain = support::recovery_chain();
    let parent =
        initialize_development_recovery(&chain.genesis, &development_state_budget()).unwrap();
    for field in 0..4 {
        let mut record = support::envelope(&chain, 1);
        match field {
            0 => {
                record.expected.height = u64::MAX;
                record.expected.application = Some(
                    eve_protocol_config::records::hash_application_commitment(
                        eve_protocol_config::records::ApplicationCommitmentInput {
                            genesis: record.expected.identity.genesis,
                            protocol_version: record.expected.identity.protocol_version,
                            execution_height: record.expected.height,
                            evm_root: record.expected.evm_root,
                            system_root: record.expected.system_root,
                            execution_hash: record.expected.execution_hash,
                        },
                    )
                    .unwrap(),
                );
            }
            1 => record.finalized.header.height = 2,
            2 => record.lookahead.frame.header.height = 1,
            3 => record.lookahead.frame.header.height = 3,
            _ => unreachable!(),
        }
        assert_eq!(
            prepare_development_recovery(
                &parent,
                Arc::new(record),
                &development_state_budget(),
                CLONE_BYTES,
            )
            .unwrap_err(),
            RecoveryError::WrongHeight,
        );
    }
    assert_eq!(recovery_state_commit(&parent), &chain.commits[0]);
}
