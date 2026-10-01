// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    solidity_probe_types::{SolidityProbeInput, SoliditySettings, SoliditySource},
    validate_solidity_probe_output::validate_solidity_probe_output,
};
use crate::provisioning::{paths::resolve_contained_path, types::ProvisionedArtifact};
use anyhow::{Result, ensure};
use std::{
    collections::BTreeMap,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

pub fn verify_solidity_probe(output: &Path, solidity: &ProvisionedArtifact) -> Result<()> {
    let input = SolidityProbeInput {
        language: "Solidity",
        sources: BTreeMap::from([(
            "B0AbiProbe.sol",
            SoliditySource {
                content: "pragma solidity 0.8.37; contract B0AbiProbe { event Deposit(address indexed recipient,uint256 amount); function deposit(address recipient) external payable { emit Deposit(recipient,msg.value); } }",
            },
        )]),
        settings: SoliditySettings {
            evm_version: "shanghai",
            output_selection: BTreeMap::from([(
                "*",
                BTreeMap::from([(
                    "*",
                    &["abi", "evm.bytecode.object"] as &'static [&'static str],
                )]),
            )]),
        },
    };
    let input_path = resolve_contained_path(output, Path::new("solidity-api-probe.json"))?;
    std::fs::write(input_path, serde_json::to_vec_pretty(&input)?)?;
    let mut child = Command::new(&solidity.executable)
        .arg("--standard-json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .ok_or_else(|| anyhow::anyhow!("compiler stdin unavailable"))?
        .write_all(&serde_json::to_vec(&input)?)?;
    let response = child.wait_with_output()?;
    std::fs::write(
        resolve_contained_path(output, Path::new("solidity-api-probe-output.json"))?,
        &response.stdout,
    )?;
    ensure!(response.status.success(), "Solidity API compile failed");
    validate_solidity_probe_output(&response.stdout)
}
