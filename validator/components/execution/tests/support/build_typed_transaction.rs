use super::support::sign_hash;
use alloy_consensus::{SignableTransaction, TxEip1559, TxEip2930, TxEnvelope};
use alloy_eips::{eip2718::Encodable2718, eip2930::AccessList};
use alloy_primitives::{Address, Bytes, TxKind, U256};

pub fn build_typed_transaction(
    transaction_type: u8,
    access_list: AccessList,
    nonce: u64,
    to: Address,
) -> Bytes {
    assert!(
        matches!(transaction_type, 1 | 2),
        "fixture supports only type 1/2"
    );
    if transaction_type == 1 {
        let transaction = TxEip2930 {
            chain_id: 31_337,
            nonce,
            gas_price: 2_000_000_000,
            gas_limit: 30_000,
            to: TxKind::Call(to),
            value: U256::from(42),
            access_list,
            input: Bytes::new(),
        };
        let signature = sign_hash(transaction.signature_hash());
        return TxEnvelope::Eip2930(transaction.into_signed(signature))
            .encoded_2718()
            .into();
    }
    let transaction = TxEip1559 {
        chain_id: 31_337,
        nonce,
        max_fee_per_gas: 3_000_000_000,
        max_priority_fee_per_gas: 500_000_000,
        gas_limit: 30_000,
        to: TxKind::Call(to),
        value: U256::from(42),
        access_list,
        input: Bytes::new(),
    };
    let signature = sign_hash(transaction.signature_hash());
    TxEnvelope::Eip1559(transaction.into_signed(signature))
        .encoded_2718()
        .into()
}
