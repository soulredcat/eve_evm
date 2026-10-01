// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
use alloy_consensus::ReceiptEnvelope;
use alloy_eips::eip2718::Decodable2718;
use alloy_primitives::{B256, TxKind, keccak256};
use anyhow::{Result, ensure};
use eve_evm::ValidatedTransaction;

pub(super) fn acceptance_transition_from_receipt(
    fixture: &AcceptanceFixture,
    transaction: &ValidatedTransaction,
    raw: &[u8],
) -> Result<Option<u8>> {
    if transaction.sender() != fixture.authority
        || transaction.evm().kind != TxKind::Call(fixture.address)
    {
        return Ok(None);
    }
    let input = transaction.evm().data.as_ref();
    let selector = keccak256(b"transition(uint8)");
    if input.len() != 36
        || input[..4] != selector[..4]
        || input[4..35].iter().any(|byte| *byte != 0)
    {
        return Ok(None);
    }
    let action = input[35];
    if !fixture.transitions.contains_key(&action) {
        return Ok(None);
    }
    let mut remaining = raw;
    let receipt = ReceiptEnvelope::decode_2718(&mut remaining)
        .map_err(|_| anyhow::anyhow!("acceptance canonical receipt decoding"))?;
    ensure!(remaining.is_empty(), "acceptance receipt trailing data");
    if !receipt.is_success() {
        return Ok(None);
    }
    let event = keccak256(b"Transition(bytes32,uint8)");
    let encoded_action = B256::from(alloy_primitives::U256::from(action).to_be_bytes::<32>());
    let mut found = false;
    for log in receipt.logs() {
        if log.address == fixture.address
            && log.topics() == [event, B256::from(fixture.digest), encoded_action]
            && log.data.data.is_empty()
        {
            ensure!(!found, "duplicate acceptance transition event");
            found = true;
        }
    }
    Ok(found.then_some(action))
}
