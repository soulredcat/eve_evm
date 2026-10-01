// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::artifacts::{validate_archive_links, validate_archive_members};

#[test]
fn rejects_traversal_foreign_absolute_and_ambiguous_archive_members() {
    for input in [
        "go/../escape",
        "/go/file",
        "foreign/file",
        "go/path\twith-tab",
        "go/path\\escape",
        "",
    ] {
        assert!(validate_archive_members(input, "go").is_err(), "{input:?}");
    }
    assert!(validate_archive_members("go/\ngo/bin/go\ngo/src/main.go\n", "go").is_ok());
    // Pinned Comet includes one ordinary workflow filename ending in a space.
    assert!(validate_archive_members("go/.github/workflows/e2e-nightly-38x.yml \n", "go").is_ok());
}

#[test]
fn permits_contained_links_and_rejects_escaping_or_special_entries() {
    let good = "lrwxrwxrwx 1/1 0 2026-09-30 00:00:00 node/bin/npm -> ../lib/npm.js\n";
    assert!(validate_archive_links(good, "node").is_ok());
    for input in [
        "lrwxrwxrwx 1/1 0 2026-09-30 00:00:00 node/bin/npm -> ../../outside\n",
        "lrwxrwxrwx 1/1 0 2026-09-30 00:00:00 node/bin/npm -> /etc/passwd\n",
        "hrw-r--r-- 1/1 0 2026-09-30 00:00:00 node/file link to other/file\n",
        "prw-r--r-- 1/1 0 2026-09-30 00:00:00 node/fifo\n",
        "broken metadata\n",
    ] {
        assert!(validate_archive_links(input, "node").is_err(), "{input:?}");
    }
}

#[cfg(target_os = "linux")]
#[test]
fn preserves_pinned_unicode_names_under_a_c_locale_without_allowing_controls() {
    use super::super::artifacts::read_archive_listing;
    use std::process::Command;
    let directory = tempfile::tempdir().unwrap();
    std::fs::create_dir(directory.path().join("go")).unwrap();
    std::fs::write(directory.path().join("go/Þfoo.go"), "package fixture\n").unwrap();
    let archive = directory.path().join("fixture.tar.gz");
    assert!(
        Command::new("tar")
            .args(["--create", "--gzip", "--file"])
            .arg(&archive)
            .arg("--directory")
            .arg(directory.path())
            .arg("go")
            .status()
            .unwrap()
            .success()
    );
    let original = Command::new("tar")
        .args(["--list", "--gzip", "--file"])
        .arg(&archive)
        .env("LC_ALL", "C")
        .output()
        .unwrap();
    assert!(original.status.success());
    assert!(validate_archive_members(&String::from_utf8(original.stdout).unwrap(), "go").is_err());
    let names = read_archive_listing(&archive, false).unwrap();
    assert!(names.contains("go/Þfoo.go"));
    assert!(validate_archive_members(&names, "go").is_ok());
    assert!(validate_archive_links(&read_archive_listing(&archive, true).unwrap(), "go").is_ok());
    std::fs::write(
        directory.path().join("go/with-tab\t.go"),
        "package fixture\n",
    )
    .unwrap();
    assert!(
        Command::new("tar")
            .args(["--create", "--gzip", "--file"])
            .arg(&archive)
            .arg("--directory")
            .arg(directory.path())
            .arg("go")
            .status()
            .unwrap()
            .success()
    );
    assert!(
        validate_archive_members(&read_archive_listing(&archive, false).unwrap(), "go").is_err()
    );
}
