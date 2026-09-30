#[cfg(target_os = "linux")]
#[test]
fn extracted_source_build_does_not_require_or_embed_enclosing_git_metadata() {
    use super::super::builds::configure_go_source_build;
    use std::process::Command;
    let go = std::env::var_os("EVE_GO_BINARY").expect("Run pinned tool provisioning first");
    let directory = tempfile::tempdir().unwrap();
    std::fs::write(
        directory.path().join("go.mod"),
        "module eve-fixture.invalid/provenance\n\ngo 1.27.1\n",
    )
    .unwrap();
    std::fs::write(
        directory.path().join("main.go"),
        "package main\nfunc main() {}\n",
    )
    .unwrap();
    assert!(
        Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(directory.path())
            .status()
            .unwrap()
            .success()
    );
    let missing_git = directory.path().join("missing-git-dir");
    let original = Command::new(&go)
        .args(["build", "-o", "original", "."])
        .current_dir(directory.path())
        .env("GOTOOLCHAIN", "local")
        .env("GIT_DIR", &missing_git)
        .output()
        .unwrap();
    assert!(!original.status.success());
    assert!(String::from_utf8_lossy(&original.stderr).contains("VCS"));
    let binary = directory.path().join("checked");
    let mut command = Command::new(&go);
    configure_go_source_build(&mut command, 1);
    let checked = command
        .args(["-o"])
        .arg(&binary)
        .arg(".")
        .current_dir(directory.path())
        .env("GIT_DIR", &missing_git)
        .output()
        .unwrap();
    assert!(
        checked.status.success(),
        "{}",
        String::from_utf8_lossy(&checked.stderr)
    );
    let identity = Command::new(&go)
        .args(["version", "-m"])
        .arg(&binary)
        .output()
        .unwrap();
    assert!(identity.status.success());
    assert!(!String::from_utf8_lossy(&identity.stdout).contains("vcs.revision="));
}
