use super::{TransactionValidationError, ValidatedTransaction};
use alloy_consensus::{Transaction, TxEnvelope, transaction::SignerRecoverable};
use alloy_eips::eip2718::{Decodable2718, Encodable2718};
use alloy_primitives::keccak256;
use revm::context::TxEnv;

/// Decode exactly the signed bytes. Reject unsupported forks and trailing data.
pub fn decode_signed_transaction(
    raw: &[u8],
    chain_id: u64,
    maximum_bytes: usize,
) -> Result<ValidatedTransaction, TransactionValidationError> {
    if raw.is_empty() {
        return Err(TransactionValidationError::Empty);
    }
    if raw.len() > maximum_bytes {
        return Err(TransactionValidationError::TooLarge);
    }
    if !matches!(raw[0], 1 | 2 | 0xc0..=0xff) {
        return Err(TransactionValidationError::UnsupportedType);
    }
    let mut remaining = raw;
    let envelope = TxEnvelope::decode_2718(&mut remaining)
        .map_err(|_| TransactionValidationError::MalformedEncoding)?;
    if !remaining.is_empty() || envelope.encoded_2718() != raw {
        return Err(TransactionValidationError::NonCanonicalEncoding);
    }
    let transaction_type = match &envelope {
        TxEnvelope::Legacy(_) => 0,
        TxEnvelope::Eip2930(_) => 1,
        TxEnvelope::Eip1559(_) => 2,
        _ => return Err(TransactionValidationError::UnsupportedType),
    };
    if envelope.chain_id() != Some(chain_id) {
        return Err(TransactionValidationError::WrongChain);
    }
    if envelope
        .max_priority_fee_per_gas()
        .is_some_and(|priority| priority > envelope.max_fee_per_gas())
    {
        return Err(TransactionValidationError::InvalidFeeCaps);
    }
    // This trait checks low-s, unlike unchecked recovery and inherent helpers.
    let sender = SignerRecoverable::recover_signer(&envelope)
        .map_err(|_| TransactionValidationError::InvalidSignature)?;
    let evm = TxEnv::builder()
        .tx_type(Some(transaction_type))
        .caller(sender)
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
        .map_err(|_| TransactionValidationError::InvalidEnvironment)?;
    Ok(ValidatedTransaction {
        hash: keccak256(raw),
        sender,
        transaction_type,
        evm,
    })
}
