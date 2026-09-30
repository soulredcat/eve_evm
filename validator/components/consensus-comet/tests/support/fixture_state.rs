use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};

use anyhow::{Context, Result};
use eve_consensus_comet::wire::tendermint::abci::ValidatorUpdate;

pub const API_TRANSACTION: &[u8] = b"EVE_B0_API_V1";

#[derive(Clone, Debug, Default)]
pub struct FixtureState {
    pub committed_height: i64,
    pub committed_hash: Vec<u8>,
    pub pending: Option<(i64, Vec<u8>)>,
    pub initial_validators: Vec<ValidatorUpdate>,
    pub finalized_hashes: BTreeMap<i64, Vec<u8>>,
    pub last_commit_power: BTreeMap<i64, i64>,
    pub operations: BTreeSet<&'static str>,
    pub transaction_count: usize,
}

pub fn load_fixture_state(path: &Path) -> Result<FixtureState> {
    if !path.exists() {
        return Ok(FixtureState::default());
    }
    let record: serde_json::Value = serde_json::from_slice(&std::fs::read(path)?)?;
    Ok(FixtureState {
        committed_height: record["height"].as_i64().context("checkpoint height")?,
        committed_hash: hex::decode(record["hash"].as_str().context("checkpoint hash")?)?,
        ..Default::default()
    })
}
