// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::{Transaction, TxEnvelope, transaction::SignerRecoverable};
use alloy_eips::eip2718::{Decodable2718, Encodable2718};
use revm::context::TxEnv;

pub fn decode_reference_transaction(raw: &[u8]) -> Result<(TxEnv, u8), &'static str> {
    let mut remaining = raw;
    let envelope = TxEnvelope::decode_2718(&mut remaining)
        .map_err(|_| "TransactionException.INVALID_SIGNATURE_VRS")?;
    if !remaining.is_empty() || envelope.encoded_2718() != raw {
        return Err("TransactionException.INVALID_SIGNATURE_VRS");
    }
    let transaction_type = match envelope {
        TxEnvelope::Legacy(_) => 0,
        TxEnvelope::Eip2930(_) => 1,
        TxEnvelope::Eip1559(_) => 2,
        _ => panic!("Pinned Shanghai corpus includes an unsupported transaction family"),
    };
    let caller = SignerRecoverable::recover_signer(&envelope)
        .map_err(|_| "TransactionException.INVALID_SIGNATURE_VRS")?;
    let transaction = TxEnv::builder()
        .tx_type(Some(transaction_type))
        .caller(caller)
        .gas_limit(envelope.gas_limit())
        .gas_price(envelope.max_fee_per_gas())
        .gas_priority_fee(envelope.max_priority_fee_per_gas())
        .kind(envelope.kind())
        .value(envelope.value())
        .data(envelope.input().clone())
        .nonce(envelope.nonce())
        .chain_id(envelope.chain_id())
        .access_list(envelope.access_list().cloned().unwrap_or_default())
        .build()
        .map_err(|_| "TransactionException.INVALID_SIGNATURE_VRS")?;
    Ok((transaction, transaction_type))
}
