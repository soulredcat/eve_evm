// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use xtask::verification::manifests::load_test_group::load_test_group;

#[test]
fn test_group_features_are_explicit_bounded_unique_cargo_names() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("group.toml");
    let prefix = "version=1\npackage='fixture'\nrequirements=['B3']\ntests=['case']\n";
    std::fs::write(&path, prefix).unwrap();
    assert!(
        load_test_group(directory.path(), "group.toml")
            .unwrap()
            .features
            .is_empty()
    );
    std::fs::write(
        &path,
        format!("{prefix}features=['development-acceptance']\n"),
    )
    .unwrap();
    assert_eq!(
        load_test_group(directory.path(), "group.toml")
            .unwrap()
            .features,
        ["development-acceptance"]
    );
    for invalid in [
        "['']",
        "['--all-features']",
        "['one','one']",
        "['a b']",
        "['one/two']",
        "['../escape']",
    ] {
        std::fs::write(&path, format!("{prefix}features={invalid}\n")).unwrap();
        assert!(
            load_test_group(directory.path(), "group.toml").is_err(),
            "{invalid}"
        );
    }
    std::fs::write(&path, format!("{prefix}features=['{}']\n", "x".repeat(65))).unwrap();
    assert!(load_test_group(directory.path(), "group.toml").is_err());
}
