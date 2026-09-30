use super::validate_provisioned_tools;
use crate::provisioning::{
    clients::{verify_client_probe, verify_solidity_probe},
    paths::resolve_contained_path,
    types::{ProvisionedTools, ToolPins},
};
use anyhow::Result;
use std::path::Path;

pub fn reuse_provisioned_tools(
    root: &Path,
    pin_path: &Path,
    output: &Path,
    pins: &ToolPins,
) -> Result<ProvisionedTools> {
    let report =
        validate_provisioned_tools(root, pin_path, &output.join("provisioned-tools.json"))?;
    let node = report
        .artifacts
        .iter()
        .find(|artifact| artifact.name == "node")
        .ok_or_else(|| anyhow::anyhow!("verified Node artifact missing"))?;
    let solidity = report
        .artifacts
        .iter()
        .find(|artifact| artifact.name == "solidity")
        .ok_or_else(|| anyhow::anyhow!("verified Solidity artifact missing"))?;
    verify_solidity_probe(output, solidity)?;
    verify_client_probe(
        &resolve_contained_path(output, Path::new("clients"))?,
        node,
        &pins.clients,
    )?;
    Ok(report)
}
