use alloy_consensus::{SignableTransaction, TxEnvelope, TxLegacy};
use alloy_eips::eip2718::Encodable2718;
use alloy_primitives::{Address, Bytes, Signature, TxKind, U256, address, keccak256};
use eve_evm::{BlockEnvironment, FeePoolAddresses, InMemoryDB};
use k256::ecdsa::SigningKey;
use revm::state::AccountInfo;

pub fn sender() -> Address {
    let key = SigningKey::from_bytes((&[7_u8; 32]).into()).unwrap();
    let public = key.verifying_key().to_encoded_point(false);
    Address::from_slice(&keccak256(&public.as_bytes()[1..])[12..])
}

pub fn environment() -> BlockEnvironment {
    BlockEnvironment {
        chain_id: 31_337,
        number: 1,
        timestamp: 1_700_000_000,
        gas_limit: 30_000_000,
        base_fee: 1_000_000_000,
        proposer: address!("00000000000000000000000000000000000000aa"),
        previous_consensus_hash: keccak256(b"disposable-consensus-parent"),
        fee_pools: FeePoolAddresses {
            node_pool: address!("000000000000000000000000000000000000f101"),
            validator_pool: address!("000000000000000000000000000000000000f102"),
        },
        maximum_transaction_bytes: 128 * 1_024,
    }
}

pub fn state() -> InMemoryDB {
    let mut state = InMemoryDB::default();
    state.insert_account_info(
        sender(),
        AccountInfo {
            balance: U256::from(10_u64).pow(U256::from(20)),
            ..Default::default()
        },
    );
    state
}

pub fn legacy(nonce: u64, kind: TxKind, value: U256, gas_limit: u64, data: Bytes) -> Bytes {
    let transaction = TxLegacy {
        chain_id: Some(31_337),
        nonce,
        gas_price: 2_000_000_000,
        gas_limit,
        to: kind,
        value,
        input: data,
    };
    let signature = sign_hash(transaction.signature_hash());
    TxEnvelope::Legacy(transaction.into_signed(signature))
        .encoded_2718()
        .into()
}

pub fn sign_hash(hash: alloy_primitives::B256) -> Signature {
    // Deliberately unsafe deterministic fixture key; never a runtime default.
    let key = SigningKey::from_bytes((&[7_u8; 32]).into()).unwrap();
    let (signature, recovery) = key.sign_prehash_recoverable(hash.as_slice()).unwrap();
    Signature::from_bytes_and_parity(&signature.to_bytes(), recovery.is_y_odd())
}
