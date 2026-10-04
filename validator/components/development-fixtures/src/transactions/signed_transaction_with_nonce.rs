// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_consensus::{SignableTransaction, TxEnvelope, TxLegacy};
use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{Address, Bytes, Signature, TxKind, U256};
use k256::ecdsa::SigningKey;

/// Publicly known unsafe CLASSICAL_DEV corpus key; never an operational credential.
pub fn signed_transaction_with_nonce(nonce: u64) -> Bytes {
    let transaction = TxLegacy {
        chain_id: Some(31_337),
        nonce,
        gas_price: 2_000_000_000,
        gas_limit: 100_000,
        to: TxKind::Call(Address::with_last_byte(0x42)),
        value: U256::ZERO,
        input: Bytes::new(),
    };
    let key = SigningKey::from_bytes((&[7_u8; 32]).into()).unwrap();
    let (signature, recovery) = key
        .sign_prehash_recoverable(transaction.signature_hash().as_slice())
        .unwrap();
    let signature = Signature::from_bytes_and_parity(&signature.to_bytes(), recovery.is_y_odd());
    TxEnvelope::Legacy(transaction.into_signed(signature))
        .encoded_2718()
        .into()
}
