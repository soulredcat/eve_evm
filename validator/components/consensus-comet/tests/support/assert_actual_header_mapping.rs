use anyhow::{Context, Result, ensure};
use eve_consensus_comet::{
    consensus::commitments::{ExecutionHeight, match_next_header_commitment},
    wire::tendermint::types::Header,
};

use super::FixtureState;

pub fn assert_actual_header_mapping(
    headers: &[serde_json::Value],
    state: &FixtureState,
) -> Result<()> {
    for height in 2..=4 {
        let record = &headers[(height - 1) as usize];
        let header = Header {
            chain_id: record["chain_id"].as_str().context("header chain")?.into(),
            height: record["height"]
                .as_str()
                .context("header height")?
                .parse()?,
            app_hash: hex::decode(record["app_hash"].as_str().context("header app hash")?)?,
            ..Default::default()
        };
        let commitment: [u8; 32] = state.finalized_hashes[&height]
            .clone()
            .try_into()
            .map_err(|_| anyhow::anyhow!("fixture application hash length"))?;
        ensure!(
            match_next_header_commitment(
                "eve-b0-api-v1",
                ExecutionHeight(height),
                &commitment,
                &header
            )
            .is_ok(),
            "real engine H/H+1 binding failed"
        );
    }
    ensure!(
        headers[0]["next_validators_hash"] == headers[0]["validators_hash"],
        "validator update changed NextValidatorsHash before H+1"
    );
    ensure!(
        headers[0]["validators_hash"] == headers[1]["validators_hash"],
        "validator update activated before H+2"
    );
    ensure!(
        headers[0]["validators_hash"] != headers[2]["validators_hash"],
        "validator update did not activate at H+2"
    );
    ensure!(
        headers[1]["next_validators_hash"] == headers[2]["validators_hash"],
        "H+1 NextValidatorsHash does not bind the H+2 set"
    );
    ensure!(
        state.last_commit_power[&4] == 10 && state.last_commit_power[&5] == 20,
        "H+3 last-commit metadata mapping failed"
    );
    Ok(())
}
