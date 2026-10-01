// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};

pub fn validate_solidity_probe_output(bytes: &[u8]) -> Result<()> {
    let result: serde_json::Value = serde_json::from_slice(bytes)?;
    if let Some(errors) = result["errors"].as_array() {
        for error in errors {
            ensure!(
                error["severity"] != "error",
                "Solidity standard-json returned compiler error"
            );
        }
    }
    let contract = &result["contracts"]["B0AbiProbe.sol"]["B0AbiProbe"];
    let abi = contract["abi"]
        .as_array()
        .context("Solidity probe ABI is missing")?;
    let bytecode = contract["evm"]["bytecode"]["object"]
        .as_str()
        .context("Solidity probe bytecode is missing")?;
    ensure!(
        abi.len() == 2 && !bytecode.is_empty(),
        "Solidity probe did not produce ABI and Shanghai bytecode"
    );
    Ok(())
}
