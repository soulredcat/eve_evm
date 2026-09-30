use xtask::verification::manifests::validate_requirement_registry::validate_requirement_registry;

#[test]
fn all_security_interop_and_persistence_requirements_are_registered_without_claiming_completion() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let cases = validate_requirement_registry(root).unwrap();
    assert_eq!(cases.len(), 48);
    for id in [
        "T-M01", "T-M10", "T-P01", "T-P10", "T-BR01", "T-BR12", "T-I01", "T-I12", "T-N09", "T-N12",
    ] {
        assert_eq!(cases[id], "NOT_IMPLEMENTED");
    }
}

#[test]
fn a_missing_or_unreviewed_required_registry_fails() {
    let fixture = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(fixture.path().join("config/gates/requirements")).unwrap();
    std::fs::write(
        fixture.path().join("config/gates/registry.toml"),
        "version = 1\ngates = []\n",
    )
    .unwrap();
    assert!(validate_requirement_registry(fixture.path()).is_err());
    std::fs::write(
        fixture
            .path()
            .join("config/gates/requirements/majority.toml"),
        "version = 1\ncases = []\n",
    )
    .unwrap();
    assert!(validate_requirement_registry(fixture.path()).is_err());
}
