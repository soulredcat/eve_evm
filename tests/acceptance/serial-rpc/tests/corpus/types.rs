// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_primitives::{Address, B256, Bytes, U256};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct FixtureAccount {
    pub nonce: U256,
    pub balance: U256,
    pub code: Bytes,
    pub storage: BTreeMap<U256, U256>,
}

#[derive(Debug, Deserialize)]
pub struct FixtureEnvironment {
    #[serde(rename = "currentCoinbase")]
    pub coinbase: Address,
    #[serde(rename = "currentGasLimit")]
    pub gas_limit: U256,
    #[serde(rename = "currentNumber")]
    pub number: U256,
    #[serde(rename = "currentTimestamp")]
    pub timestamp: U256,
    #[serde(rename = "currentBaseFee")]
    pub base_fee: U256,
    #[serde(rename = "currentRandom")]
    pub random: B256,
}

#[derive(Debug, Deserialize)]
pub struct FixtureConfiguration {
    pub chainid: U256,
}

#[derive(Debug, Deserialize)]
pub struct FixtureReceipt {
    pub rlp: Bytes,
}

#[derive(Debug, Deserialize)]
pub struct FixturePost {
    pub hash: B256,
    pub logs: B256,
    pub txbytes: Bytes,
    pub state: BTreeMap<Address, FixtureAccount>,
    pub receipt: Option<FixtureReceipt>,
    #[serde(rename = "expectException")]
    pub expected_exception: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FixtureCase {
    pub env: FixtureEnvironment,
    pub pre: BTreeMap<Address, FixtureAccount>,
    pub post: BTreeMap<String, Vec<FixturePost>>,
    pub config: FixtureConfiguration,
}

#[derive(Deserialize)]
pub struct CorpusInventory {
    pub schema_version: u32,
    pub source_revision: String,
    pub archive_sha256: String,
    pub target_fork: String,
    pub files: BTreeMap<String, String>,
}
