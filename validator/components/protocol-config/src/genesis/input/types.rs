// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, Bytes, U256};
use serde::Deserialize;

use crate::genesis::GenesisError;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DevelopmentSpecInput {
    pub schema_version: u32,
    pub protocol_version: u32,
    pub network_name: String,
    pub evm_chain_id: u64,
    pub initial_timestamp: u64,
    pub profile: String,
    pub accounts: Vec<DevelopmentAccountInput>,
    pub validators: Vec<DevelopmentValidatorInput>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DevelopmentAccountInput {
    pub address: Address,
    pub funded_balance: U256,
    pub nonce: u64,
    pub code: Bytes,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DevelopmentValidatorInput {
    pub owner: Address,
    pub classical_public_key: String,
    pub self_bond: U256,
    pub voting_power: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DevelopmentSpecError {
    TooLarge,
    MalformedInput,
    UnsupportedProfile,
    InvalidPublicKey,
    InvalidGenesis(GenesisError),
}
