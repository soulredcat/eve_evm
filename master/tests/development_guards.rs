use eve_master::development::config::{
    resolve_development_directory::resolve_development_directory,
    validate_development_mode::validate_development_mode,
};
use std::{path::Path, process::Command};

#[test]
fn local_producer_requires_exact_development_mode_and_acknowledgement() {
    assert!(validate_development_mode("DEV_ALL_IN_ONE", true).is_ok());
    for (mode, acknowledged) in [
        ("DEV_ALL_IN_ONE", false),
        ("MASTER_SYNC_ONLY", true),
        ("PRODUCTION", true),
        ("", true),
    ] {
        assert!(validate_development_mode(mode, acknowledged).is_err());
    }
}

#[test]
fn development_namespace_rejects_escape_and_nonignored_data_before_creation() {
    let fixture = tempfile::tempdir().unwrap();
    assert!(
        Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(fixture.path())
            .status()
            .unwrap()
            .success()
    );
    std::fs::write(fixture.path().join(".gitignore"), "/local-tests/\n").unwrap();
    let root = fixture.path().canonicalize().unwrap();
    let expected = root.join("local-tests/harness");
    assert_eq!(
        resolve_development_directory(&root, Path::new("local-tests/harness")).unwrap(),
        expected
    );
    assert!(!expected.exists());
    for path in [
        "master/production",
        "local-tests",
        "local-tests/../../outside",
    ] {
        assert!(
            resolve_development_directory(&root, Path::new(path)).is_err(),
            "{path}"
        );
    }
    let outside = tempfile::tempdir().unwrap();
    assert!(resolve_development_directory(&root, outside.path()).is_err());
}

#[cfg(unix)]
#[test]
fn development_namespace_rejects_symlink_escape() {
    let fixture = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    std::fs::create_dir(fixture.path().join("local-tests")).unwrap();
    std::os::unix::fs::symlink(outside.path(), fixture.path().join("local-tests/linked")).unwrap();
    assert!(
        resolve_development_directory(fixture.path(), Path::new("local-tests/linked/database"))
            .is_err()
    );
}
