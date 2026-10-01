// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use sha2::{Digest, Sha256};
use std::{
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub(in crate::development::acceptance::enabled::tests) fn compile_transition_contract(
    solc: &Path,
    output: &Path,
) -> Result<Vec<u8>> {
    // Explicit source artifact keeps the maintained contract in one canonical owner package.
    let source_path = PathBuf::from(
        std::env::var_os("EVE_B3_TRANSITION_SOURCE")
            .context("pinned transition source artifact required")?,
    );
    let metadata = std::fs::symlink_metadata(&source_path)?;
    ensure!(
        metadata.is_file() && metadata.len() <= 65_536,
        "transition source artifact bounds"
    );
    let source = std::fs::read(&source_path)?;
    ensure!(
        source.len() <= 65_536
            && hex::encode(Sha256::digest(&source))
                == "ff2c7d9fadd98ed863033f686d1b90b374d0fd9ba34a2260859fdebabd9ce644",
        "maintained transition source identity mismatch"
    );
    let source = std::str::from_utf8(&source)?;
    let input = serde_json::json!({"language":"Solidity","sources":{"AcceptanceValidatorTransitions.sol":{"content":source}},"settings":{"evmVersion":"shanghai","optimizer":{"enabled":true,"runs":200},"outputSelection":{"*":{"*":["evm.deployedBytecode.object"]}}}});
    let mut child = Command::new(solc)
        .arg("--standard-json")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    child
        .stdin
        .take()
        .context("compiler input unavailable")?
        .write_all(&serde_json::to_vec(&input)?)?;
    let result = child.wait_with_output()?;
    ensure!(
        result.status.success() && result.stdout.len() <= 4 * 1_048_576,
        "pinned transition compile failed"
    );
    std::fs::write(output, &result.stdout)?;
    let parsed: serde_json::Value = serde_json::from_slice(&result.stdout)?;
    ensure!(
        !parsed["errors"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|error| error["severity"] == "error"),
        "transition compiler reported source error"
    );
    let object =
        parsed["contracts"]["AcceptanceValidatorTransitions.sol"]["AcceptanceValidatorTransitions"]
            ["evm"]["deployedBytecode"]["object"]
            .as_str()
            .context("transition deployed code unavailable")?;
    let bytes = hex::decode(object)?;
    ensure!(
        !bytes.is_empty() && bytes.len() <= 24_576,
        "transition deployed byte bounds"
    );
    Ok(bytes)
}
