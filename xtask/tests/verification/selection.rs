// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use xtask::verification::manifests::{
    load_gate_selection::load_gate_selection, load_test_group::load_test_group,
};

#[test]
fn unsupported_gates_remain_pending_and_empty_groups_fail() {
    let fixture = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(fixture.path().join("config/gates")).unwrap();
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    std::fs::copy(
        repository.join("config/gates/registry.toml"),
        fixture.path().join("config/gates/registry.toml"),
    )
    .unwrap();
    let (gates, pending) = load_gate_selection(fixture.path(), &["B5".into()], false).unwrap();
    assert!(gates.is_empty() && pending.len() == 1);
    assert!(load_gate_selection(fixture.path(), &["unknown".into()], false).is_err());
    assert!(load_gate_selection(fixture.path(), &[], false).is_err());
    assert!(load_gate_selection(fixture.path(), &[], true).is_err());
    std::fs::write(
        fixture.path().join("config/gates/group.toml"),
        "version = 1\npackage = 'fixture'\nrequirements = []\ntests = []\n",
    )
    .unwrap();
    assert!(load_test_group(fixture.path(), "config/gates/group.toml").is_err());
}

#[test]
fn gate_manifest_identity_and_local_path_cannot_be_substituted() {
    let fixture = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(fixture.path().join("config/gates")).unwrap();
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    std::fs::copy(
        repository.join("config/gates/registry.toml"),
        fixture.path().join("config/gates/registry.toml"),
    )
    .unwrap();
    std::fs::write(fixture.path().join("config/gates/B0.toml"), "version = 1\nid = 'SEC0'\nprofile = 'fixture'\ngroups = ['group.toml']\nrequired_inputs = []\n").unwrap();
    assert!(load_gate_selection(fixture.path(), &["B0".into()], false).is_err());
    assert!(load_test_group(fixture.path(), "../outside.toml").is_err());
}

#[test]
fn b0_selection_requires_its_core_security_subgate_manifest() {
    let fixture = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(fixture.path().join("config/gates")).unwrap();
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    for file in ["registry.toml", "B0.toml", "SEC0.toml"] {
        std::fs::copy(
            repository.join("config/gates").join(file),
            fixture.path().join("config/gates").join(file),
        )
        .unwrap();
    }
    let (gates, pending) = load_gate_selection(fixture.path(), &["B0".into()], false).unwrap();
    assert_eq!(
        gates
            .iter()
            .map(|gate| gate.id.as_str())
            .collect::<Vec<_>>(),
        ["B0", "SEC0"]
    );
    assert!(pending.is_empty());
    std::fs::remove_file(fixture.path().join("config/gates/SEC0.toml")).unwrap();
    assert!(load_gate_selection(fixture.path(), &["B0".into()], false).is_err());
}
