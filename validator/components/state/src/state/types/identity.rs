// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::B256;
use eve_protocol_config::{
    network::SecurityProfile,
    records::{
        ApplicationCommitment, EvmStateRoot, ExecutionBlockHash, GenesisHash, SystemStateRoot,
    },
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateIdentity {
    pub genesis: GenesisHash,
    pub network_name: String,
    pub evm_chain_id: u64,
    pub protocol_version: u32,
    pub security_profile: SecurityProfile,
    pub key_epoch: u64,
    pub configuration_digest: B256,
}

/// Local execution version. Construction alone proves neither finality nor durability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateVersion {
    pub identity: StateIdentity,
    pub height: u64,
    pub timestamp: u64,
    pub execution_hash: ExecutionBlockHash,
    pub evm_root: EvmStateRoot,
    pub system_root: SystemStateRoot,
    pub application: Option<ApplicationCommitment>,
    /// Local complete-data binding, including code/history outside consensus roots.
    pub content_digest: B256,
}
