// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{
    structure::checking::check_structure::check_structure,
    verification::{
        artifacts::digest_inputs::digest_inputs,
        commands::require_command_success::require_command_success,
        manifests::{load_gate_selection::load_gate_selection, load_test_group::load_test_group},
        types::report_types::VerificationReport,
    },
};
use anyhow::{Result, ensure};
use std::{collections::BTreeSet, path::Path};

pub(super) fn run_selected_gates(
    root: &Path,
    artifacts: &Path,
    report: &mut VerificationReport,
    all: bool,
) -> Result<()> {
    super::record_git_identity::record_git_identity(root, artifacts, report)?;
    let (digest, inputs) = digest_inputs(root)?;
    report.source_sha256 = digest;
    report.inputs = inputs;
    let structure = check_structure(root, Path::new("config/structure-policy.toml"))?;
    std::fs::write(
        artifacts.join("structure.json"),
        serde_json::to_vec_pretty(&structure)?,
    )?;
    ensure!(
        structure.violations.is_empty(),
        "Mandatory structure gate failed: {:?}",
        structure.violations
    );
    let (gates, pending) = load_gate_selection(root, &report.requested, all)?;
    report.pending = pending;
    if !gates.is_empty() {
        report.registered_requirements = crate::verification::manifests::validate_requirement_registry::validate_requirement_registry(root)?;
        let registry: crate::verification::types::manifest_types::GateRegistry = toml::from_str(
            &std::fs::read_to_string(root.join("config/gates/registry.toml"))?,
        )?;
        report.core_requirement_status =
            crate::verification::manifests::validate_core_mapping::validate_core_mapping(
                root, &registry,
            )?;
    }
    ensure!(
        report.pending.is_empty(),
        "Requested coverage is NOT_IMPLEMENTED: {:?}",
        report.pending
    );
    let ownership = crate::publication::ownership::check_ownership::check_ownership(root)?;
    std::fs::write(
        artifacts.join("ownership.json"),
        serde_json::to_vec_pretty(&ownership)?,
    )?;
    ensure!(
        ownership.violations.is_empty(),
        "Mandatory Redcat ownership gate failed: {:?}",
        ownership.violations
    );
    let mut groups = BTreeSet::new();
    for gate in gates {
        for path in gate.required_inputs {
            ensure!(
                report.inputs.contains_key(&path),
                "Required source/config input is absent: {path}"
            );
        }
        groups.extend(gate.groups);
        report
            .profile
            .push_str(&format!("; {}={}", gate.id, gate.profile));
    }
    let cargo = env!("CARGO");
    super::record_tool_identity::record_tool_identity(root, report)?;
    super::record_native_environment::record_native_environment(root, artifacts, report)?;
    let version = require_command_success(root, artifacts, report, cargo, &["--version"])?;
    ensure!(
        version.starts_with("cargo 1.97.1 "),
        "Unpinned Cargo version: {version}"
    );
    let rustc = std::path::Path::new(cargo)
        .with_file_name(format!("rustc{}", std::env::consts::EXE_SUFFIX));
    report.compiler_identity = require_command_success(
        root,
        artifacts,
        report,
        rustc
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Compiler path must be UTF-8"))?,
        &["--version", "--verbose"],
    )?;
    ensure!(
        report
            .compiler_identity
            .lines()
            .any(|line| line == "release: 1.97.1")
            && report
                .compiler_identity
                .lines()
                .any(|line| line == "commit-hash: 8bab26f4f68e0e26f0bb7960be334d5b520ea452"),
        "Compiler release/revision differs from pin"
    );
    require_command_success(
        root,
        artifacts,
        report,
        cargo,
        &["fmt", "--all", "--", "--check"],
    )?;
    require_command_success(
        root,
        artifacts,
        report,
        cargo,
        &[
            "clippy",
            "--locked",
            "--workspace",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    if groups.contains("config/gates/groups/serial-rpc-b2.toml") {
        crate::verification::fixtures::prepare_b2_acceptance::prepare_b2_acceptance(
            root, artifacts, report,
        )?;
    } else if groups.contains("config/gates/groups/master-b1.toml") {
        require_command_success(
            root,
            artifacts,
            report,
            cargo,
            &["build", "--locked", "-p", "eve-master"],
        )?;
    }
    if groups.contains("config/gates/groups/validator-consensus-b3.toml") {
        crate::verification::fixtures::prepare_b3_acceptance::prepare_b3_acceptance(
            root, artifacts, report,
        )?;
    }
    for path in groups {
        super::run_test_group::run_test_group(
            root,
            artifacts,
            report,
            load_test_group(root, &path)?,
        )?;
    }
    require_command_success(
        root,
        artifacts,
        report,
        cargo,
        &["build", "--locked", "--workspace", "--release"],
    )?;
    let (after, _) = digest_inputs(root)?;
    ensure!(
        after == report.source_sha256,
        "First-party source changed during verification; rerun the gate"
    );
    Ok(())
}
