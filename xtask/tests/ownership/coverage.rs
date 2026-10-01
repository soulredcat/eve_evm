// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{Fixture, REDCAT, annotation, notice};

#[test]
fn reports_all_four_ownership_categories() {
    let fixture = Fixture::new();
    fixture.write("public/src/main.rs", &notice("//", "fn main() {}\n"));
    fixture.write("validator/src/main.rs", &notice("//", "fn main() {}\n"));
    fixture.write("master/src/main.rs", &notice("//", "fn main() {}\n"));
    fixture.write("public/config.json", "{}\n");
    let upstream = "validator/components/authentication/tests/fixtures/nist-acvp/prompt.json";
    fixture.write(upstream, "{}\n");
    fixture.pin(upstream, "{}\n");
    fixture.write(
        "LICENSES/LicenseRef-NIST-ACVP-Upstream.txt",
        "Preserved upstream test terms.\n",
    );
    fixture.reuse(
        &(annotation("public/config.json", REDCAT, "2026 Redcat")
            + &annotation(upstream, "LicenseRef-NIST-ACVP-Upstream", "NOASSERTION")),
    );
    let report = fixture.pass();
    assert_eq!(report.coverage.inline_first_party, 6);
    assert_eq!(report.coverage.annotated_first_party, 1);
    assert_eq!(report.coverage.upstream, 1);
    assert_eq!(report.coverage.license_text, 3);
    assert_eq!(report.files.len(), 11);
}

#[test]
fn scans_tracked_and_new_sources_but_ignores_local_artifacts() {
    let fixture = Fixture::new();
    fixture.write("tracked.rs", &notice("//", "pub fn first() {}\n"));
    assert!(
        std::process::Command::new("git")
            .args(["add", "tracked.rs"])
            .current_dir(fixture.root())
            .status()
            .unwrap()
            .success()
    );
    fixture.write("new.rs", &notice("//", "pub fn second() {}\n"));
    fixture.write("local-tests/secret.txt", "Unpublishable local content\n");
    let report = fixture.pass();
    assert!(report.files.iter().any(|entry| entry.path == "tracked.rs"));
    assert!(report.files.iter().any(|entry| entry.path == "new.rs"));
    assert!(
        !report
            .files
            .iter()
            .any(|entry| entry.path.starts_with("local-tests/"))
    );
}

#[test]
fn absent_required_policy_fails_without_inventing_coverage() {
    let fixture = Fixture::new();
    std::fs::remove_file(fixture.root().join("REUSE.toml")).unwrap();
    assert!(
        xtask::publication::ownership::check_ownership::check_ownership(fixture.root()).is_err()
    );
}

#[test]
fn rejects_unsupported_reuse_policy_version() {
    let fixture = Fixture::new();
    fixture.write("REUSE.toml", &notice("#", "version = 2\n"));
    fixture.reject("Unsupported ownership REUSE policy version");
}
