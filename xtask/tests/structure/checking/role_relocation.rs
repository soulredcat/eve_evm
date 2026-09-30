use crate::support::Fixture;
use std::{fs, path::Path};
use xtask::structure::inspection::compute_source_digest::compute_source_digest;

#[test]
fn t_l06_current_execution_source_relocation_preserves_exact_root_and_receipt_code() {
    let repository = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let canonical = "validator/components/execution/src";
    let relocated = "validator/components/execution-relocated/src";
    let fixture = Fixture::new();
    let mut paths = Vec::new();
    collect_rust_files(&repository.join(canonical), &mut paths);
    paths.sort();
    assert!(
        paths
            .iter()
            .any(|path| path.ends_with("compute_state_root.rs"))
    );
    assert!(paths.iter().any(|path| path.ends_with("build_receipt.rs")));
    for absolute in &paths {
        let suffix = absolute
            .strip_prefix(repository.join(canonical))
            .unwrap()
            .to_string_lossy()
            .replace('\\', "/");
        let original = format!("{canonical}/{suffix}");
        let copy = format!("{relocated}/{suffix}");
        fixture.write(&copy, &fs::read_to_string(absolute).unwrap());
        assert_eq!(
            compute_source_digest(repository, &original).unwrap(),
            compute_source_digest(fixture.root(), &copy).unwrap(),
            "canonical source bytes diverged: {suffix}",
        );
    }
    fixture.policy(&format!("[[adapters]]\npath = \"{relocated}/execution/serial/fees/handler/fee_handler_adapter.rs\"\nexternal_trait = \"revm::handler::Handler\"\nreason = \"Exact canonical fee-adapter copy for source-relocation regression.\"\nreviewer = \"test-reviewer\"\n"));
    fixture.assert_pass();
}

#[test]
fn t_l06_role_fixture_builds_without_private_master_and_rejects_transitive_master_edge() {
    let fixture = Fixture::new();
    fixture.write(
        "Cargo.toml",
        "[workspace]\nresolver = \"3\"\nmembers = [\"public\", \"validator\"]\n",
    );
    for (role, dependency, component) in [
        ("public", "public-recovery-fixture", "recovery"),
        ("validator", "validator-execution-fixture", "execution"),
    ] {
        fixture.write(&format!("{role}/Cargo.toml"), &format!("[package]\nname = \"{role}-role-fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\n{dependency} = {{ path = \"components/{component}\" }}\n"));
        fixture.write(&format!("{role}/src/main.rs"), "fn main() {}\n");
        fixture.write(
            &format!("{role}/components/{component}/Cargo.toml"),
            &format!(
                "[package]\nname = \"{dependency}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"
            ),
        );
        fixture.write(
            &format!("{role}/components/{component}/src/lib.rs"),
            "pub struct FinalizedHeight(pub u64);\n",
        );
        fixture.write(
            &format!("{role}/components/{component}/README.md"),
            "Dependency-graph fixture only; no node runtime or finality implementation.\n",
        );
    }
    fixture.assert_pass();
    assert!(!fixture.root().join("master").exists());
    for manifest in ["public/Cargo.toml", "validator/Cargo.toml"] {
        let result = fixture.cargo("", &["check", "--offline", "--manifest-path", manifest]);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    fixture.write(
        "master/Cargo.toml",
        "[package]\nname = \"private-master-fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    fixture.write("master/src/lib.rs", "pub struct PrivateArchive;\n");
    fixture.write("public/components/recovery/Cargo.toml", "[package]\nname = \"public-recovery-fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\nprivate-master-fixture = { path = \"../../../master\" }\n");
    let report = fixture.assert_rejected();
    assert!(
        report
            .violations
            .iter()
            .any(|message| message.contains("master"))
    );
}

#[test]
fn t_l06_forbidden_root_component_directories_fail() {
    for path in [
        "crates/encoding/src/encode_height.rs",
        "create/encoding/src/encode_height.rs",
    ] {
        let fixture = Fixture::new();
        fixture.write(path, "fn encode_height() {}\n");
        let report = fixture.assert_rejected();
        assert!(
            report
                .violations
                .iter()
                .any(|message| message.contains("role placement"))
        );
    }
}

fn collect_rust_files(directory: &Path, paths: &mut Vec<std::path::PathBuf>) {
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_rust_files(&path, paths);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            paths.push(path);
        }
    }
}
