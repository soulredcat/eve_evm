// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::{SignableTransaction, TxEnvelope, TxLegacy};
use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{Address, Bytes, Signature, TxKind, U256, keccak256};
use k256::ecdsa::SigningKey;

pub fn ephemeral_signer() -> (SigningKey, Address) {
    let key = SigningKey::random(&mut k256::elliptic_curve::rand_core::OsRng);
    let public = key.verifying_key().to_encoded_point(false);
    let hash = keccak256(&public.as_bytes()[1..]);
    (key, Address::from_slice(&hash.as_slice()[12..]))
}

pub fn sign_legacy(
    key: &SigningKey,
    nonce: u64,
    to: Address,
    gas_limit: u64,
    value: U256,
) -> Bytes {
    let transaction = TxLegacy {
        chain_id: Some(31_337),
        nonce,
        gas_price: 2_000_000_000,
        gas_limit,
        to: TxKind::Call(to),
        value,
        input: Bytes::new(),
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
    TxEnvelope::Legacy(transaction.into_signed(signature))
        .encoded_2718()
        .into()
}
