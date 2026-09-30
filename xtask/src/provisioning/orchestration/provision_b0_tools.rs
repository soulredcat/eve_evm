use super::{
    reuse_provisioned_tools::reuse_provisioned_tools, run_provision_plan,
    write_provisioned_report::write_provisioned_report,
};
use crate::provisioning::{
    paths::{prepare_output_directory, resolve_contained_path},
    pins::load_tool_pins,
    types::ProvisionedTools,
};
use anyhow::{Context, Result, ensure};
use std::path::Path;

/// Provision the complete B0 tool/API fixture without changing global toolchains.
pub fn provision_b0_tools(
    root: &Path,
    pins: &Path,
    output: &Path,
    jobs: usize,
) -> Result<ProvisionedTools> {
    ensure!(
        cfg!(target_os = "linux") && cfg!(target_arch = "x86_64"),
        "tool provisioning requires Linux x86_64"
    );
    ensure!((1..=16).contains(&jobs), "build jobs must be in 1..=16");
    let pin_path = resolve_contained_path(root, pins)?;
    let pins = load_tool_pins(root, pins)?;
    let output = prepare_output_directory(root, output)?;
    let lock_path = output.join("provision.lock");
    let lock = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&lock_path)
        .context("provisioning already active or stale task lock exists; inspect before retry")?;
    let result = if output.join("provisioned-tools.json").exists() {
        reuse_provisioned_tools(
            root,
            pin_path.strip_prefix(root.canonicalize()?)?,
            &output,
            &pins,
        )
    } else {
        run_provision_plan(root, &output, &pins, jobs)
            .and_then(|report| write_provisioned_report(&pin_path, &output, report))
    };
    drop(lock);
    std::fs::remove_file(lock_path).context("cannot remove this task's provisioning lock")?;
    result
}
