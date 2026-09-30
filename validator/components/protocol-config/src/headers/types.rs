use alloy_primitives::{Address, B256, Bloom};

use crate::records::{EvmStateRoot, ExecutionBlockHash, GenesisHash};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionHeaderInput {
    pub parent: ExecutionBlockHash,
    pub beneficiary: Address,
    pub state_root: EvmStateRoot,
    pub transaction_root: B256,
    pub receipt_root: B256,
    pub logs_bloom: Bloom,
    pub number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub parent_timestamp: u64,
    pub previous_consensus_hash: B256,
    pub base_fee: u64,
    pub protocol_version: u32,
    pub genesis: GenesisHash,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HeaderError {
    InvalidEnvironment,
}
