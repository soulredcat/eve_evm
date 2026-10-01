// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "catalog/core_mapping.rs"]
mod core_mapping;

use std::path::{Path, PathBuf};
use xtask::verification::{
    manifests::validate_gate_catalog::validate_gate_catalog,
    types::manifest_types::{GateRegistration, GateRegistry},
};

#[test]
fn complete_fifteen_gate_catalog_preserves_all_current_core_security_bulks() {
    let registry = registry();
    validate_gate_catalog(&registry).unwrap();
    assert_eq!(registry.gates.len(), 15);
    for id in ["B11", "SEC1", "SEC3"] {
        assert!(
            !registry
                .gates
                .iter()
                .find(|gate| gate.id == id)
                .unwrap()
                .implemented
        );
    }
}

#[test]
fn deferred_programs_cannot_be_added_to_core_or_block_secure_capacity() {
    for id in ["SEC2", "INT0", "INT1", "INT2", "INT3"] {
        let mut changed = registry();
        changed.gates.push(GateRegistration {
            id: id.into(),
            manifest: String::new(),
            implemented: false,
            prerequisites: Vec::new(),
        });
        assert!(validate_gate_catalog(&changed).is_err(), "{id}");
    }
    let registry = registry();
    for gate in registry
        .gates
        .iter()
        .filter(|gate| ["SEC3", "B10", "B11"].contains(&gate.id.as_str()))
    {
        assert!(
            gate.prerequisites
                .iter()
                .all(|id| !id.starts_with("INT") && id != "SEC2")
        );
    }
}

#[test]
fn omitted_duplicate_or_substituted_mandatory_gates_cannot_shrink_all_coverage() {
    let mut omitted = registry();
    omitted.gates.retain(|gate| gate.id != "B11");
    assert!(validate_gate_catalog(&omitted).is_err());
    let mut duplicated = registry();
    duplicated.gates.push(GateRegistration {
        id: "B0".into(),
        manifest: "config/gates/B0.toml".into(),
        implemented: true,
        prerequisites: Vec::new(),
    });
    assert!(validate_gate_catalog(&duplicated).is_err());
    let mut substituted = registry();
    substituted
        .gates
        .iter_mut()
        .find(|gate| gate.id == "SEC3")
        .unwrap()
        .id = "OPTIONAL_CORE_SECURITY".into();
    assert!(validate_gate_catalog(&substituted).is_err());
}

#[test]
fn unknown_self_duplicate_and_cyclic_gate_dependencies_fail_closed() {
    for (owner, target) in [
        ("B1", "MASTER_OVERRIDE"),
        ("B1", "B1"),
        ("B1", "B0"),
        ("B0", "B11"),
    ] {
        let mut registry = registry();
        registry
            .gates
            .iter_mut()
            .find(|gate| gate.id == owner)
            .unwrap()
            .prerequisites
            .push(target.into());
        assert!(
            validate_gate_catalog(&registry).is_err(),
            "{owner} -> {target}"
        );
    }
}

#[test]
fn removing_a_required_core_security_or_secure_capacity_dependency_is_rejected() {
    for (owner, dependency) in [("B10", "SEC3"), ("SEC3", "SEC1"), ("SEC1", "B3")] {
        let mut registry = registry();
        registry
            .gates
            .iter_mut()
            .find(|gate| gate.id == owner)
            .unwrap()
            .prerequisites
            .retain(|id| id != dependency);
        assert!(validate_gate_catalog(&registry).is_err());
    }
}

fn repository() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn registry() -> GateRegistry {
    toml::from_str(
        &std::fs::read_to_string(repository().join("config/gates/registry.toml")).unwrap(),
    )
    .unwrap()
}
