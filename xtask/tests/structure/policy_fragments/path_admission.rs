// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{FRAGMENT, assert_load_rejected, setup};
use std::process::Command;

#[test]
fn fragment_paths_reject_absolute_escape_components_missing_files_and_nonfiles() {
    for relative in [
        "/tmp/adapters.toml",
        "C:/adapters.toml",
        "../adapters.toml",
        "config/../adapters.toml",
        "config//adapters.toml",
        "config/./adapters.toml",
        "config\\adapters.toml",
        "config/missing.toml",
        "config/structure",
        "config/structure/readme.txt",
        "config/blocked/child.toml",
    ] {
        let fixture = setup();
        fixture.write(
            "config/structure/readme.txt",
            "version = 1\nadapters = []\n",
        );
        fixture.write(
            "config/blocked",
            "A regular file, not a parent directory.\n",
        );
        fixture.policy(&format!(
            "adapter_files = [\"{}\"]\n",
            relative.replace('\\', "\\\\")
        ));
        assert_load_rejected(&fixture);
    }
}

#[test]
fn ignored_fragment_rejects_even_when_it_was_already_tracked_before_ignore_rule() {
    for tracked in [false, true] {
        let fixture = setup();
        if tracked {
            let output = Command::new("git")
                .args(["add", "--", FRAGMENT])
                .current_dir(fixture.root())
                .output()
                .unwrap();
            assert!(output.status.success());
        }
        fixture.write(".gitignore", "/target/\n/config/structure/\n");
        assert_load_rejected(&fixture);
    }
}

#[cfg(unix)]
#[test]
fn fragment_file_and_parent_symlinks_cannot_import_external_policy() {
    use std::fs;
    use std::os::unix::fs::symlink;
    let external = tempfile::tempdir().unwrap();
    fs::write(
        external.path().join("adapters.toml"),
        "version = 1\nadapters = []\n",
    )
    .unwrap();
    let fixture = setup();
    symlink(
        external.path().join("adapters.toml"),
        fixture.root().join("config/structure/linked.toml"),
    )
    .unwrap();
    fixture.policy("adapter_files = [\"config/structure/linked.toml\"]\n");
    assert_load_rejected(&fixture);
    let fixture = setup();
    symlink(external.path(), fixture.root().join("config/linked")).unwrap();
    fixture.policy("adapter_files = [\"config/linked/adapters.toml\"]\n");
    assert_load_rejected(&fixture);
}
