use anyhow::Result;
use eve_state::StateVersion;
use serde::Serialize;

#[derive(Serialize)]
struct LocalStatus<'a> {
    mode: &'static str,
    security_scope: &'static str,
    genesis: String,
    network: &'a str,
    height: u64,
    execution_hash: String,
    evm_root: String,
    system_root: String,
    authenticated_validator_finality: bool,
}

pub fn print_development_status(version: &StateVersion) -> Result<()> {
    let status = LocalStatus {
        mode: "DEV_ALL_IN_ONE",
        security_scope: "LOCAL_DURABLE_ONLY_NO_VALIDATOR_FINALITY",
        genesis: format!("{:#x}", version.identity.genesis.0),
        network: &version.identity.network_name,
        height: version.height,
        execution_hash: format!("{:#x}", version.execution_hash.0),
        evm_root: format!("{:#x}", version.evm_root.0),
        system_root: format!("{:#x}", version.system_root.0),
        authenticated_validator_finality: false,
    };
    println!("{}", serde_json::to_string_pretty(&status)?);
    Ok(())
}
