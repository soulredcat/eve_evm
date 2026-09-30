use super::generate_client_probe::generate_client_probe;
use crate::provisioning::{
    execution::{run_checked_command, verify_tool_version},
    paths::resolve_contained_path,
    types::{ClientPins, ProvisionedArtifact},
};
use anyhow::{Result, ensure};
use std::{path::Path, process::Command};

pub fn verify_client_probe(
    directory: &Path,
    node: &ProvisionedArtifact,
    pins: &ClientPins,
) -> Result<()> {
    let compiler = resolve_contained_path(directory, Path::new("node_modules/typescript/bin/tsc"))?;
    verify_tool_version(
        &node.executable,
        &[
            compiler
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("compiler path must be UTF8"))?,
            "--version",
        ],
        &format!("Version {}", pins.typescript),
    )?;
    generate_client_probe(directory)?;
    run_checked_command(
        Command::new(&node.executable)
            .arg(&compiler)
            .current_dir(directory)
            .args([
                "--strict",
                "--target",
                "ES2022",
                "--module",
                "NodeNext",
                "--moduleResolution",
                "NodeNext",
                "--outDir",
                "probe-output",
                "probe.ts",
            ]),
        directory,
        "typescript-api-probe",
    )?;
    let script = resolve_contained_path(directory, Path::new("probe-output/probe.js"))?;
    let result = Command::new(&node.executable)
        .arg(script)
        .current_dir(directory)
        .output()?;
    ensure!(
        result.status.success()
            && String::from_utf8(result.stdout)?.trim() == "EVE_B0_CLIENT_API_OK",
        "typed client/ABI execution probe failed"
    );
    Ok(())
}
