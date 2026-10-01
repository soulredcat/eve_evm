// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::BTreeMap;

use alloy_primitives::{Address, B256, Bytes, U256};
use eve_protocol_config::records::{ExecutionBlockHash, SystemRecord};

use super::StateIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StateAccount {
    pub nonce: u64,
    pub balance: U256,
    pub code_hash: B256,
    /// Complete nonzero storage; a missing slot is known zero in this materialization.
    pub storage: BTreeMap<U256, U256>,
}

/// Complete materialized data, not an authenticated capability. Call validation
/// before using external data, and match computed roots to the trusted version.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompleteState {
    pub identity: StateIdentity,
    pub accounts: BTreeMap<Address, StateAccount>,
    pub codes: BTreeMap<B256, Bytes>,
    pub system: BTreeMap<B256, SystemRecord>,
    pub block_hashes: BTreeMap<u64, ExecutionBlockHash>,
}
