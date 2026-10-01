// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::{SignableTransaction, TxEnvelope, TxLegacy};
use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{Address, Bytes, Signature, TxKind, U256};
use k256::ecdsa::SigningKey;

pub(in crate::development::acceptance::enabled::tests) fn signed_transaction(
    key: &SigningKey,
    nonce: u64,
    to: Address,
    gas_limit: u64,
    input: Bytes,
) -> Bytes {
    let transaction = TxLegacy {
        chain_id: Some(31_337),
        nonce,
        gas_price: 2_000_000_000,
        gas_limit,
        to: TxKind::Call(to),
        value: U256::ZERO,
        input,
    };
    let (signature, recovery) = key
        .sign_prehash_recoverable(transaction.signature_hash().as_slice())
        .unwrap();
    let signature = Signature::from_bytes_and_parity(&signature.to_bytes(), recovery.is_y_odd());
    TxEnvelope::Legacy(transaction.into_signed(signature))
        .encoded_2718()
        .into()
}
