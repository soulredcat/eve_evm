use crate::support::{Fixture, plain_policy};
use std::{fs, path::Path};
use xtask::structure::{
    inspection::inspect_file::inspect_file, policy::validate_policy::validate_policy,
};

#[test]
fn t_l04_wildcard_traversal_absolute_and_stale_registrations_fail() {
    for path in [
        "docs/**/*.md",
        "docs/?.md",
        "../outside.md",
        "/outside.md",
        "docs/./encoding.md",
        "docs//encoding.md",
        "docs\\encoding.md",
    ] {
        let mut policy = plain_policy();
        policy
            .size_reviews
            .push(xtask::structure::types::policy_types::SizeReview {
                path: path.into(),
                lines: 201,
                reason: "Exact repository paths only".into(),
                reviewer: "test-reviewer".into(),
            });
        let violations = validate_policy(&policy, &[path.into()]);
        assert!(
            violations
                .iter()
                .any(|message| message.contains("exact repository-relative path")),
            "invalid policy path: {path}; {violations:?}",
        );
    }
    let fixture = Fixture::new();
    fixture.policy("[[size_reviews]]\npath = \"docs/missing.md\"\nlines = 201\nreason = \"Stale review fixture.\"\nreviewer = \"test-reviewer\"\n");
    let report = fixture.assert_rejected();
    assert!(
        report
            .violations
            .iter()
            .any(|message| message.contains("Stale"))
    );
}

#[test]
fn duplicate_registrations_within_one_category_fail() {
    let fixture = Fixture::new();
    fixture.write("Cargo.lock", "# Lockfile fixture.\n");
    let exclusion = "[[exclusions]]\npath = \"Cargo.lock\"\nkind = \"generated\"\nsource = \"Cargo\"\ngenerator = \"Cargo 1.97.1\"\nreason = \"Generated dependency identities.\"\n";
    fixture.policy(&format!("{exclusion}{exclusion}"));
    let report = fixture.assert_rejected();
    assert!(
        report
            .violations
            .iter()
            .any(|message| message.contains("Duplicate"))
    );
}

#[test]
fn inspection_rejects_paths_resolving_outside_the_repository() {
    let fixture = Fixture::new();
    let outside = tempfile::tempdir().unwrap();
    let source = outside.path().join("encode_height.rs");
    fs::write(&source, "fn encode_height() {}\n").unwrap();
    let result = inspect_file(fixture.root(), source.to_str().unwrap(), &plain_policy());
    assert!(
        result.is_err(),
        "absolute external source escaped repository ownership"
    );
}

#[test]
fn inspection_rejects_a_linked_parent_directory() {
    let fixture = Fixture::new();
    let outside = tempfile::tempdir().unwrap();
    fs::write(
        outside.path().join("encode_height.rs"),
        "fn encode_height() {}\n",
    )
    .unwrap();
    let link = fixture.root().join("linked");
    link_directory(outside.path(), &link);
    let result = inspect_file(fixture.root(), "linked/encode_height.rs", &plain_policy());
    assert!(
        result.is_err(),
        "linked directory escaped repository ownership"
    );
}

#[cfg(unix)]
fn link_directory(target: &Path, link: &Path) {
    std::os::unix::fs::symlink(target, link).unwrap();
}

#[cfg(windows)]
fn link_directory(target: &Path, link: &Path) {
    let output = std::process::Command::new("cmd")
        .args(["/d", "/c", "mklink", "/J"])
        .arg(link)
        .arg(target)
        .output()
        .expect("temporary junction creation");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
