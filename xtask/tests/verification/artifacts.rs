use std::process::Command;
use xtask::verification::{
    artifacts::create_artifact_directory::create_artifact_directory,
    orchestration::run_verify::run_verify,
};

#[test]
fn artifact_creation_requires_git_ignore_and_records_failed_future_gate() {
    let fixture = tempfile::tempdir().unwrap();
    assert!(
        Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(fixture.path())
            .status()
            .unwrap()
            .success()
    );
    assert!(create_artifact_directory(fixture.path()).is_err());
    std::fs::write(fixture.path().join(".gitignore"), "/local-tests/\n").unwrap();
    std::fs::create_dir_all(fixture.path().join("config/gates")).unwrap();
    std::fs::write(
        fixture.path().join("config/structure-policy.toml"),
        "version = 1\ncurrent_bulk = 0\n",
    )
    .unwrap();
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    std::fs::copy(
        repository.join("config/gates/registry.toml"),
        fixture.path().join("config/gates/registry.toml"),
    )
    .unwrap();
    assert!(
        Command::new("git")
            .args(["add", "."])
            .current_dir(fixture.path())
            .status()
            .unwrap()
            .success()
    );
    assert!(
        Command::new("git")
            .args([
                "-c",
                "user.name=Disposable fixture",
                "-c",
                "user.email=fixture@example.invalid",
                "commit",
                "--quiet",
                "-m",
                "Create gate fixture"
            ])
            .current_dir(fixture.path())
            .status()
            .unwrap()
            .success()
    );
    assert!(run_verify(fixture.path(), vec!["B2".into()], false).is_err());
    let directories: Vec<_> = std::fs::read_dir(fixture.path().join("local-tests"))
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    let report_path = directories
        .iter()
        .map(|path| path.join("report.json"))
        .find(|path| path.exists())
        .unwrap();
    let report: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&report_path).unwrap()).unwrap();
    assert_eq!(report["status"], "FAIL");
    assert_eq!(report["test_count"], 0);
    assert_eq!(report["pending"].as_array().unwrap().len(), 1);
    assert!(
        report_path
            .parent()
            .unwrap()
            .join("structure.json")
            .exists()
    );
}
