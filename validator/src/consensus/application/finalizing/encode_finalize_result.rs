// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::receipt_cumulative_gas;
use crate::consensus::application::context::native_application_hash;
use crate::development::acceptance::{AcceptanceFixture, acceptance_validator_updates};
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::{ExecTxResult, ResponseFinalizeBlock};
use eve_evm::decode_signed_transaction;
use eve_state::{BlockPayload, StateVersion};

/// Valid EVM reverts retain ABCI code zero and their canonical failed receipt.
pub(in crate::consensus::application) fn encode_finalize_result(
    version: &StateVersion,
    block: &BlockPayload,
    fixture: Option<&AcceptanceFixture>,
) -> Result<ResponseFinalizeBlock> {
    ensure!(
        block.transactions.len() == block.receipts.len(),
        "retained execution receipt count mismatch"
    );
    let mut previous_gas = 0_u64;
    let mut results = Vec::with_capacity(block.transactions.len());
    for (raw, receipt) in block.transactions.iter().zip(&block.receipts) {
        let validated = decode_signed_transaction(raw, version.identity.evm_chain_id, 131_072)
            .map_err(|_| anyhow::anyhow!("retained canonical envelope invalid"))?;
        let cumulative = receipt_cumulative_gas(receipt)?;
        let used = cumulative
            .checked_sub(previous_gas)
            .ok_or_else(|| anyhow::anyhow!("retained gas ordering invalid"))?;
        previous_gas = cumulative;
        results.push(ExecTxResult {
            code: 0,
            data: receipt.to_vec(),
            gas_wanted: i64::try_from(validated.evm().gas_limit)?,
            gas_used: i64::try_from(used)?,
            ..Default::default()
        });
    }
    ensure!(
        previous_gas == block.header.gas_used,
        "retained cumulative gas differs from header"
    );
    Ok(ResponseFinalizeBlock {
        validator_updates: acceptance_validator_updates(fixture, version, block)?,
        tx_results: results,
        app_hash: native_application_hash(version)?,
        ..Default::default()
    })
}
