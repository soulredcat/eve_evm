// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    AcceptanceFixture, acceptance_transition_from_receipt::acceptance_transition_from_receipt,
};
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::ValidatorUpdate;
use eve_evm::decode_signed_transaction;
use eve_state::{BlockPayload, StateVersion};
use std::collections::BTreeSet;

pub(crate) fn acceptance_validator_updates(
    fixture: Option<&AcceptanceFixture>,
    version: &StateVersion,
    block: &BlockPayload,
) -> Result<Vec<ValidatorUpdate>> {
    let Some(fixture) = fixture else {
        return Ok(Vec::new());
    };
    ensure!(
        block.transactions.len() == block.receipts.len(),
        "acceptance receipt count mismatch"
    );
    let mut seen = BTreeSet::new();
    let mut updates = Vec::new();
    for (raw, receipt) in block.transactions.iter().zip(&block.receipts) {
        let transaction = decode_signed_transaction(raw, version.identity.evm_chain_id, 131_072)
            .map_err(|_| anyhow::anyhow!("acceptance retained signed transaction invalid"))?;
        if let Some(action) = acceptance_transition_from_receipt(fixture, &transaction, receipt)? {
            ensure!(
                seen.insert(action),
                "repeated acceptance action in one decided block"
            );
            updates.extend_from_slice(&fixture.transitions[&action]);
        }
    }
    ensure!(updates.len() <= 4, "acceptance validator-update capacity");
    Ok(updates)
}
