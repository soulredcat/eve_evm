// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use xtask::verification::{
    manifests::load_test_group::load_test_group,
    orchestration::cargo_test_arguments::cargo_test_arguments,
};

const GROUP_PREFIX: &str = "version=1\npackage='fixture'\nrequirements=['B4']\ntests=['case']\ndoc_tests=['doc_case']\nfeatures=['feature-one']\n";

#[test]
fn manifest_default_profile_is_test_and_explicit_true_selects_release_for_both_suites() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("group.toml");
    for (field, expected) in [
        ("", false),
        ("release=false\n", false),
        ("release=true\n", true),
    ] {
        std::fs::write(&path, format!("{GROUP_PREFIX}{field}")).unwrap();
        let group = load_test_group(directory.path(), "group.toml").unwrap();
        assert_eq!(group.release, expected);
        assert_eq!(group.tests, ["case"]);
        assert_eq!(group.doc_tests, ["doc_case"]);
        let features = group.features.join(",");
        for docs in [false, true] {
            let arguments = cargo_test_arguments(&group.package, &features, docs, group.release);
            assert_eq!(arguments.contains(&"--release"), expected);
            assert_eq!(arguments.contains(&"--doc"), docs);
            assert_eq!(arguments.contains(&"--all-targets"), !docs);
        }
    }
}

#[test]
fn manifest_release_requires_a_typed_boolean_and_cannot_override_program_or_arguments() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("group.toml");
    for invalid in [
        "'true'",
        "'false'",
        "1",
        "0",
        "['true']",
        "{ enabled=true }",
    ] {
        std::fs::write(&path, format!("{GROUP_PREFIX}release={invalid}\n")).unwrap();
        assert!(
            load_test_group(directory.path(), "group.toml").is_err(),
            "{invalid}"
        );
    }
    for forbidden in [
        "program='other-cargo'",
        "arguments=['--skip','case']",
        "environment={ RUST_TEST_THREADS='0' }",
    ] {
        std::fs::write(&path, format!("{GROUP_PREFIX}release=true\n{forbidden}\n")).unwrap();
        assert!(
            load_test_group(directory.path(), "group.toml").is_err(),
            "{forbidden}"
        );
    }
}
