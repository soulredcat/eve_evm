use crate::{
    structure::inspection::resolve_source_path::resolve_source_path,
    verification::types::manifest_types::{GateManifest, GateRegistry},
};
use anyhow::{Context, Result, ensure};
use std::{collections::BTreeSet, path::Path};

pub fn load_gate_selection(
    root: &Path,
    requested: &[String],
    all: bool,
) -> Result<(Vec<GateManifest>, Vec<String>)> {
    let registry: GateRegistry = toml::from_str(&std::fs::read_to_string(resolve_source_path(
        root,
        "config/gates/registry.toml",
    )?)?)?;
    ensure!(
        registry.version == 1 && !registry.gates.is_empty(),
        "Unsupported or empty gate registry"
    );
    super::validate_gate_catalog::validate_gate_catalog(&registry)?;
    let mut seen = BTreeSet::new();
    for gate in &registry.gates {
        ensure!(
            seen.insert(&gate.id) && !gate.id.trim().is_empty(),
            "Duplicate or empty gate registration"
        );
        for prerequisite in &gate.prerequisites {
            ensure!(
                registry.gates.iter().any(|entry| entry.id == *prerequisite),
                "Unknown prerequisite {prerequisite}"
            );
        }
    }
    ensure!(
        all || requested.len() == 1,
        "Select exactly one core, security or interop gate, or --all"
    );
    let ids = if all {
        registry.gates.iter().map(|gate| gate.id.clone()).collect()
    } else if requested == ["B0"] {
        vec!["B0".into(), "SEC0".into(), "INT0".into()]
    } else {
        requested.to_vec()
    };
    let mut manifests = Vec::new();
    let mut pending = Vec::new();
    for id in ids {
        let gate = registry
            .gates
            .iter()
            .find(|gate| gate.id == id)
            .with_context(|| format!("Unknown gate {id}"))?;
        if !gate.implemented {
            pending.push(format!(
                "{id}: NOT_IMPLEMENTED; prerequisites {:?}",
                gate.prerequisites
            ));
            continue;
        }
        let manifest: GateManifest = toml::from_str(&std::fs::read_to_string(
            resolve_source_path(root, &gate.manifest)?,
        )?)?;
        ensure!(
            manifest.version == 1
                && manifest.id == id
                && !manifest.groups.is_empty()
                && !manifest.profile.trim().is_empty(),
            "Invalid gate manifest {id}"
        );
        manifests.push(manifest);
    }
    Ok((manifests, pending))
}
