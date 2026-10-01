// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{emitter_code, execute_fixture, fixture, transition_input};
use crate::development::acceptance::enabled::acceptance_validator_updates;
use alloy_consensus::ReceiptEnvelope;
use alloy_eips::eip2718::Decodable2718;

#[test]
fn successful_spoof_events_require_authorized_sender_and_exact_transition_abi() {
    for unauthorized in [true, false] {
        let mut material = fixture();
        let digest = material.load().digest;
        let contract = material
            .genesis
            .accounts
            .iter_mut()
            .find(|account| account.address == material.contract)
            .unwrap();
        let footer = contract.code[contract.code.len() - 72..].to_vec();
        contract.code = emitter_code(&digest, &footer, 1);
        let capability = material.load();
        let mut input = transition_input(1).to_vec();
        if !unauthorized {
            input[0] ^= 1;
        }
        let key = if unauthorized {
            &material.unauthorized
        } else {
            &material.authority
        };
        let prepared = execute_fixture(&material, key, &[input.into()]);
        let receipt =
            ReceiptEnvelope::decode_2718(&mut prepared.commit.block.receipts[0].as_ref()).unwrap();
        assert!(receipt.is_success());
        assert_eq!(receipt.logs().len(), 1);
        assert!(
            acceptance_validator_updates(
                Some(&capability),
                &prepared.commit.target,
                &prepared.commit.block
            )
            .unwrap()
            .is_empty()
        );
    }
}

#[test]
fn wrong_digest_and_duplicate_real_evm_events_cannot_authorize_updates() {
    for duplicate in [false, true] {
        let mut material = fixture();
        let mut digest = material.load().digest;
        if !duplicate {
            digest[0] ^= 1;
        }
        let contract = material
            .genesis
            .accounts
            .iter_mut()
            .find(|account| account.address == material.contract)
            .unwrap();
        let footer = contract.code[contract.code.len() - 72..].to_vec();
        contract.code = emitter_code(&digest, &footer, if duplicate { 2 } else { 1 });
        let capability = material.load();
        let prepared = execute_fixture(&material, &material.authority, &[transition_input(1)]);
        let receipt =
            ReceiptEnvelope::decode_2718(&mut prepared.commit.block.receipts[0].as_ref()).unwrap();
        assert!(receipt.is_success());
        assert_eq!(receipt.logs().len(), if duplicate { 2 } else { 1 });
        let updates = acceptance_validator_updates(
            Some(&capability),
            &prepared.commit.target,
            &prepared.commit.block,
        );
        if duplicate {
            assert!(updates.is_err());
        } else {
            assert!(updates.unwrap().is_empty());
        }
    }
}
