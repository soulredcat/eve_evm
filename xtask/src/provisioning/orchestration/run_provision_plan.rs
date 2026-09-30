use super::build_tool_environment;
use crate::provisioning::{
    builds::{provision_comet, provision_openssl, provision_prebuilt},
    clients::{
        compute_client_tree_digest, install_locked_clients, verify_client_probe,
        verify_solidity_probe,
    },
    paths::resolve_contained_path,
    types::{ProvisionedTools, ToolPins},
};
use anyhow::Result;
use std::path::Path;

pub fn run_provision_plan(
    root: &Path,
    output: &Path,
    pins: &ToolPins,
    jobs: usize,
) -> Result<ProvisionedTools> {
    let go = provision_prebuilt(output, "go", &pins.go, &["version"])?;
    let comet = provision_comet(output, &pins.comet, &go, jobs)?;
    let openssl = provision_openssl(output, &pins.openssl, jobs)?;
    let solidity = provision_prebuilt(output, "solidity", &pins.solidity, &["--version"])?;
    verify_solidity_probe(output, &solidity)?;
    let node = provision_prebuilt(output, "node", &pins.node, &["--version"])?;
    let clients = install_locked_clients(root, output, &node, &pins.clients)?;
    verify_client_probe(&clients, &node, &pins.clients)?;
    let runtime_artifacts = resolve_contained_path(
        root,
        Path::new("local-tests/consensus-b0/provisioned-fixtures"),
    )?;
    std::fs::create_dir_all(&runtime_artifacts)?;
    let artifacts = vec![go, comet, openssl, solidity, node];
    let environment = build_tool_environment(root, output, &artifacts)?;
    Ok(ProvisionedTools {
        version: 1,
        pin_file_sha256: String::new(),
        client_lock_sha256: pins.clients.lock_sha256.clone(),
        client_tree_sha256: compute_client_tree_digest(&clients.join("node_modules"))?,
        platform: pins.platform.clone(),
        artifacts,
        environment,
    })
}
