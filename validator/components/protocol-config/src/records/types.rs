use alloy_primitives::{Address, B256, Bytes, U256};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GenesisHash(pub B256);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExecutionBlockHash(pub B256);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EvmStateRoot(pub B256);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SystemStateRoot(pub B256);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicationCommitment(pub B256);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ConsensusBlockId(pub B256);
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SnapshotId(pub B256);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ApplicationCommitmentInput {
    pub genesis: GenesisHash,
    pub protocol_version: u32,
    pub execution_height: u64,
    pub evm_root: EvmStateRoot,
    pub system_root: SystemStateRoot,
    pub execution_hash: ExecutionBlockHash,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SystemNamespace {
    Validator = 1,
    Fee = 2,
    Reward = 3,
    Parameter = 4,
    Task = 5,
    Evidence = 6,
    Upgrade = 7,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SystemValue {
    Validator {
        owner: Address,
        key: [u8; 32],
        power: u64,
        activation: u64,
        removal: Option<u64>,
        key_epoch: u64,
    },
    Fee {
        burned: U256,
        node_pool: U256,
        validator_pool: U256,
    },
    Reward {
        owner: Address,
        role: u8,
        liability: U256,
        index: U256,
    },
    Parameter {
        name: Bytes,
        value: Bytes,
    },
    Task {
        epoch: u64,
        task_id: B256,
        node: Address,
        content: B256,
        request_nonce: u64,
        deadline_height: u64,
        work_units: u64,
        consumed: bool,
    },
    Evidence {
        evidence_id: B256,
        offense_height: u64,
        applied: bool,
    },
    Upgrade {
        old_version: u32,
        new_version: u32,
        activation_height: u64,
        code_digest: B256,
        migration: Bytes,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemRecord {
    pub schema_version: u32,
    pub namespace: SystemNamespace,
    pub logical_key: Bytes,
    pub value: SystemValue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeltaOperation {
    pub namespace: SystemNamespace,
    pub logical_key: Bytes,
    pub value: Option<SystemRecord>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecordError {
    DuplicateKey,
    InvalidVersion,
    InvalidKey,
    InvalidRecord,
    NonCanonical,
}
