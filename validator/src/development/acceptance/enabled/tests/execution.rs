// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{execute_fixture, fixture, transition_input};
use crate::development::acceptance::enabled::acceptance_validator_updates;
use alloy_consensus::ReceiptEnvelope;
use alloy_eips::eip2718::Decodable2718;

#[test]
fn real_signed_evm_events_project_exact_updates_and_retained_replay_identically() {
    let material = fixture();
    let capability = material.load();
    let prepared = execute_fixture(
        &material,
        &material.authority,
        &[
            transition_input(1),
            transition_input(2),
            transition_input(3),
        ],
    );
    for raw in &prepared.commit.block.receipts {
        let receipt = ReceiptEnvelope::decode_2718(&mut raw.as_ref()).unwrap();
        assert!(receipt.is_success());
        assert_eq!(receipt.logs().len(), 1);
    }
    let expected = (1..=3)
        .flat_map(|action| capability.transitions[&action].clone())
        .collect::<Vec<_>>();
    let updates = acceptance_validator_updates(
        Some(&capability),
        &prepared.commit.target,
        &prepared.commit.block,
    )
    .unwrap();
    assert_eq!(updates, expected);
    assert_eq!(updates.len(), 4);
    let retained_version = prepared.commit.target.clone();
    let retained_block = prepared.commit.block.clone();
    assert_eq!(
        acceptance_validator_updates(Some(&capability), &retained_version, &retained_block)
            .unwrap(),
        updates
    );
    assert_eq!(prepared.commit.target.height, 1);
    assert!(prepared.commit.target.application.is_some());
}

#[test]
fn unauthorized_real_evm_call_reverts_without_validator_updates() {
    let material = fixture();
    let capability = material.load();
    let prepared = execute_fixture(&material, &material.unauthorized, &[transition_input(1)]);
    let receipt =
        ReceiptEnvelope::decode_2718(&mut prepared.commit.block.receipts[0].as_ref()).unwrap();
    assert!(!receipt.is_success());
    assert!(receipt.logs().is_empty());
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

#[test]
fn repeated_and_out_of_order_contract_actions_revert_after_first_authorized_event() {
    let material = fixture();
    let capability = material.load();
    let prepared = execute_fixture(
        &material,
        &material.authority,
        &[
            transition_input(1),
            transition_input(1),
            transition_input(3),
        ],
    );
    let statuses = prepared
        .commit
        .block
        .receipts
        .iter()
        .map(|raw| {
            ReceiptEnvelope::decode_2718(&mut raw.as_ref())
                .unwrap()
                .is_success()
        })
        .collect::<Vec<_>>();
    assert_eq!(statuses, vec![true, false, false]);
    assert_eq!(
        acceptance_validator_updates(
            Some(&capability),
            &prepared.commit.target,
            &prepared.commit.block
        )
        .unwrap(),
        capability.transitions[&1]
    );
}

#[test]
fn successful_actual_evm_call_without_event_cannot_create_updates() {
    let mut material = fixture();
    let contract = material
        .genesis
        .accounts
        .iter_mut()
        .find(|account| account.address == material.contract)
        .unwrap();
    let footer = contract.code[contract.code.len() - 72..].to_vec();
    let mut code = vec![0x00]; // A canonical STOP-only test account executes successfully and emits no log.
    code.extend_from_slice(&footer);
    contract.code = code.into();
    let capability = material.load();
    let prepared = execute_fixture(&material, &material.authority, &[transition_input(1)]);
    let receipt =
        ReceiptEnvelope::decode_2718(&mut prepared.commit.block.receipts[0].as_ref()).unwrap();
    assert!(receipt.is_success());
    assert!(receipt.logs().is_empty());
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

#[test]
fn corrupted_retained_receipts_and_duplicate_transition_projection_fail_closed() {
    let material = fixture();
    let capability = material.load();
    let prepared = execute_fixture(&material, &material.authority, &[transition_input(1)]);
    let mut corrupt = prepared.commit.block.clone();
    let mut bytes = corrupt.receipts[0].to_vec();
    bytes.push(0);
    corrupt.receipts[0] = bytes.into();
    assert!(
        acceptance_validator_updates(Some(&capability), &prepared.commit.target, &corrupt).is_err()
    );
    let mut duplicate = prepared.commit.block.clone();
    duplicate
        .transactions
        .push(duplicate.transactions[0].clone());
    duplicate.receipts.push(duplicate.receipts[0].clone());
    assert!(
        acceptance_validator_updates(Some(&capability), &prepared.commit.target, &duplicate)
            .is_err()
    );
    assert!(
        acceptance_validator_updates(None, &prepared.commit.target, &prepared.commit.block)
            .unwrap()
            .is_empty()
    );
}
