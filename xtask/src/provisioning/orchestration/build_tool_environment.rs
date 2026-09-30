use crate::provisioning::{paths::resolve_contained_path, types::ProvisionedArtifact};
use anyhow::{Result, ensure};
use std::{collections::BTreeMap, path::Path};

pub fn build_tool_environment(
    root: &Path,
    output: &Path,
    artifacts: &[ProvisionedArtifact],
) -> Result<BTreeMap<String, String>> {
    let mut environment = BTreeMap::new();
    for (name, variable) in [
        ("go", "EVE_GO_BINARY"),
        ("comet", "COMETBFT_BINARY"),
        ("openssl", "EVE_OPENSSL"),
        ("solidity", "EVE_SOLC_BINARY"),
        ("node", "EVE_NODE_BINARY"),
    ] {
        let matching: Vec<_> = artifacts
            .iter()
            .filter(|artifact| artifact.name == name)
            .collect();
        ensure!(
            matching.len() == 1,
            "provision report must have exactly one {name} artifact"
        );
        environment.insert(
            variable.to_owned(),
            matching[0].executable.display().to_string(),
        );
        if name == "comet" {
            environment.insert(
                "COMETBFT_SHA256".to_owned(),
                matching[0].executable_sha256.clone(),
            );
        }
    }
    let runtime_artifacts = resolve_contained_path(
        root,
        Path::new("local-tests/consensus-b0/provisioned-fixtures"),
    )?;
    let compiler =
        resolve_contained_path(output, Path::new("clients/node_modules/typescript/bin/tsc"))?;
    environment.insert("EVE_TSC_SCRIPT".to_owned(), compiler.display().to_string());
    environment.insert(
        "EVE_B0_LOCAL_ARTIFACT_DIR".to_owned(),
        runtime_artifacts.display().to_string(),
    );
    Ok(environment)
}
