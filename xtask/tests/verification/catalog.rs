use std::path::{Path, PathBuf};
use xtask::verification::{
    manifests::{
        validate_core_mapping::validate_core_mapping, validate_gate_catalog::validate_gate_catalog,
    },
    types::manifest_types::{GateRegistration, GateRegistry},
};

#[test]
fn complete_twenty_gate_catalog_preserves_all_core_security_and_interop_bulks() {
    let registry = registry();
    validate_gate_catalog(&registry).unwrap();
    assert_eq!(registry.gates.len(), 20);
    for id in ["B11", "SEC3", "INT3"] {
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
        .find(|gate| gate.id == "INT3")
        .unwrap()
        .id = "OPTIONAL_INTEROP".into();
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
fn removing_a_required_secure_capacity_or_interop_dependency_is_rejected() {
    for (owner, dependency) in [("B10", "SEC3"), ("SEC2", "SEC1"), ("INT1", "SEC2")] {
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

#[test]
fn complete_core_mapping_keeps_every_runtime_requirement_unachieved_by_foundations() {
    let statuses = validate_core_mapping(&repository(), &registry()).unwrap();
    assert_eq!(statuses.len(), 12);
    assert!(statuses.values().all(|status| status == "NOT_ACHIEVED"));
}

#[test]
fn missing_core_ids_full_test_families_and_owning_bulks_cannot_be_omitted() {
    for field in ["id", "required_tests", "owner_gates"] {
        let (fixture, mut mapping) = mapping_fixture();
        let cases = mapping.get_mut("cases").unwrap().as_array_mut().unwrap();
        if field == "id" {
            cases.pop();
        } else {
            let target = cases
                .iter_mut()
                .find(|case| case.get("id").unwrap().as_str() == Some("R12"))
                .unwrap();
            target.get_mut(field).unwrap().as_array_mut().unwrap().pop();
        }
        std::fs::write(
            fixture.path().join("config/gates/requirements/core.toml"),
            toml::to_string_pretty(&mapping).unwrap(),
        )
        .unwrap();
        assert!(
            validate_core_mapping(fixture.path(), &registry()).is_err(),
            "{field}"
        );
    }
}

#[test]
fn foundation_mapping_cannot_claim_runtime_acceptance_or_reference_a_missing_group() {
    for (field, value) in [
        ("status", "ACHIEVED"),
        ("foundation_groups", "config/gates/groups/missing.toml"),
    ] {
        let (fixture, mut mapping) = mapping_fixture();
        let case = mapping
            .get_mut("cases")
            .unwrap()
            .as_array_mut()
            .unwrap()
            .first_mut()
            .unwrap();
        if field == "status" {
            case[field] = value.into();
        } else {
            case[field] = toml::Value::Array(vec![value.into()]);
        }
        std::fs::write(
            fixture.path().join("config/gates/requirements/core.toml"),
            toml::to_string_pretty(&mapping).unwrap(),
        )
        .unwrap();
        assert!(validate_core_mapping(fixture.path(), &registry()).is_err());
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

fn mapping_fixture() -> (tempfile::TempDir, toml::Value) {
    let fixture = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(fixture.path().join("config/gates/requirements")).unwrap();
    std::fs::create_dir_all(fixture.path().join("config/gates/groups")).unwrap();
    let mapping =
        std::fs::read_to_string(repository().join("config/gates/requirements/core.toml")).unwrap();
    for entry in std::fs::read_dir(repository().join("config/gates/groups")).unwrap() {
        let entry = entry.unwrap();
        std::fs::copy(
            entry.path(),
            fixture
                .path()
                .join("config/gates/groups")
                .join(entry.file_name()),
        )
        .unwrap();
    }
    std::fs::write(
        fixture.path().join("config/gates/requirements/core.toml"),
        &mapping,
    )
    .unwrap();
    assert_eq!(
        validate_core_mapping(fixture.path(), &registry())
            .unwrap()
            .len(),
        12
    );
    (fixture, toml::from_str(&mapping).unwrap())
}
