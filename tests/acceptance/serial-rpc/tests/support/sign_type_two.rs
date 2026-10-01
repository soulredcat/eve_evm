// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::{SignableTransaction, TxEip1559, TxEnvelope};
use alloy_eips::{eip2718::Encodable2718, eip2930::AccessList};
use alloy_primitives::{Address, Bytes, Signature, TxKind, U256};
use k256::ecdsa::SigningKey;

pub fn sign_type_two(
    key: &SigningKey,
    nonce: u64,
    to: Address,
    fee: u128,
    tip: u128,
    value: U256,
) -> Bytes {
    let transaction = TxEip1559 {
        chain_id: 31_337,
        nonce,
        gas_limit: 21_000,
        max_fee_per_gas: fee,
        max_priority_fee_per_gas: tip,
        to: TxKind::Call(to),
        value,
        input: Bytes::new(),
        access_list: AccessList::default(),
    };
    let (signature, recovery) = key
        .sign_prehash_recoverable(transaction.signature_hash().as_slice())
        .unwrap();
    let bytes = signature.to_bytes();
    let signature = Signature::new(
        U256::from_be_slice(&bytes[..32]),
        U256::from_be_slice(&bytes[32..]),
        recovery.is_y_odd(),
    );
    TxEnvelope::Eip1559(transaction.into_signed(signature))
        .encoded_2718()
        .into()
}
