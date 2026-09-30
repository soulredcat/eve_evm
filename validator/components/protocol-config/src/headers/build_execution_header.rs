use alloy_consensus::{
    Header,
    constants::{EMPTY_OMMER_ROOT_HASH, EMPTY_WITHDRAWALS},
};
use alloy_primitives::{B64, Bytes, U256};

use super::{ExecutionHeaderInput, HeaderError};

pub fn build_execution_header(input: &ExecutionHeaderInput) -> Result<Header, HeaderError> {
    if input.protocol_version == 0
        || input.gas_limit == 0
        || input.gas_used > input.gas_limit
        || input.timestamp < input.parent_timestamp
        || input.base_fee == 0
    {
        return Err(HeaderError::InvalidEnvironment);
    }
    let mut extra_data = input.protocol_version.to_be_bytes().to_vec();
    extra_data.extend_from_slice(&input.genesis.0.as_slice()[..28]);
    Ok(Header {
        parent_hash: input.parent.0,
        ommers_hash: EMPTY_OMMER_ROOT_HASH,
        beneficiary: input.beneficiary,
        state_root: input.state_root.0,
        transactions_root: input.transaction_root,
        receipts_root: input.receipt_root,
        logs_bloom: input.logs_bloom,
        difficulty: U256::ZERO,
        number: input.number,
        gas_limit: input.gas_limit,
        gas_used: input.gas_used,
        timestamp: input.timestamp,
        extra_data: Bytes::from(extra_data),
        mix_hash: input.previous_consensus_hash,
        nonce: B64::ZERO,
        base_fee_per_gas: Some(input.base_fee),
        withdrawals_root: Some(EMPTY_WITHDRAWALS),
        ..Default::default()
    })
}
