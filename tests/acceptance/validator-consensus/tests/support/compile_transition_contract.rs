// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use std::{
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

pub fn compile_transition_contract(solc: &Path, output: &Path) -> Result<Vec<u8>> {
    let source = include_str!("../fixtures/AcceptanceValidatorTransitions.sol");
    let input = serde_json::json!({
        "language": "Solidity",
        "sources": {
            "AcceptanceValidatorTransitions.sol": { "content": source }
        },
        "settings": {
            "evmVersion": "shanghai",
            "optimizer": { "enabled": true, "runs": 200 },
            "outputSelection": {
                "*": { "*": ["evm.deployedBytecode.object", "abi"] }
            }
        }
    });
    let mut child = Command::new(solc)
        .arg("--standard-json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .context("compiler stdin missing")?
        .write_all(&serde_json::to_vec(&input)?)?;
    let result = child.wait_with_output()?;
    ensure!(
        result.status.success() && result.stdout.len() <= 4 * 1_048_576,
        "pinned Solidity compiler failed or oversized output"
    );
    std::fs::write(output, &result.stdout)?;
    let parsed: serde_json::Value = serde_json::from_slice(&result.stdout)?;
    ensure!(
        !parsed["errors"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|error| error["severity"] == "error"),
        "Solidity fixture compile error"
    );
    let object =
        parsed["contracts"]["AcceptanceValidatorTransitions.sol"]["AcceptanceValidatorTransitions"]
            ["evm"]["deployedBytecode"]["object"]
            .as_str()
            .context("compiled transition runtime missing")?;
    let bytes = hex::decode(object)?;
    ensure!(
        !bytes.is_empty() && bytes.len() <= 24_576,
        "invalid compiled transition runtime bounds"
    );
    Ok(bytes)
}
