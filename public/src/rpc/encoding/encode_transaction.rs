// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::{
    TxEnvelope,
    transaction::{Recovered, TransactionInfo},
};
use alloy_eips::eip2718::Decodable2718;
use alloy_primitives::Bytes;
use anyhow::{Result, ensure};
use eve_state::Header;
use serde_json::Value;
pub(crate) fn encode_transaction(
    raw: &Bytes,
    chain_id: u64,
    header: Option<&Header>,
    index: Option<u64>,
) -> Result<Value> {
    let validated = eve_evm::decode_signed_transaction(raw, chain_id, 131_072)
        .map_err(|e| anyhow::anyhow!("invalid retained signature: {e:?}"))?;
    let mut bytes = raw.as_ref();
    let envelope = TxEnvelope::decode_2718(&mut bytes)?;
    ensure!(bytes.is_empty(), "trailing transaction bytes");
    let transaction = alloy_rpc_types_eth::Transaction::from_transaction(
        Recovered::new_unchecked(envelope, validated.sender()),
        TransactionInfo {
            block_hash: header.map(Header::hash_slow),
            block_number: header.map(|header| header.number),
            index,
            base_fee: header.and_then(|header| header.base_fee_per_gas),
            block_timestamp: header.map(|header| header.timestamp),
            ..Default::default()
        },
    );
    Ok(serde_json::to_value(transaction)?)
}
