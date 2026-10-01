// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Fixture;

#[test]
fn t_l06_ignored_local_manifest_cannot_hide_a_transitive_master_dependency() {
    let fixture = Fixture::new();
    fixture.write(".gitignore", "/target/\n/public/components/hidden/\n");
    fixture.write("public/Cargo.toml", "[package]\nname = \"public-fixture\"\nversion = \"0.1.0\"\n[dependencies]\nhidden = { path = \"components/hidden\" }\n");
    fixture.write("public/src/lib.rs", "pub struct PublicRole;\n");
    fixture.write("public/components/hidden/Cargo.toml", "[package]\nname = \"hidden\"\nversion = \"0.1.0\"\n[dependencies]\narchive = { path = \"../../../master\" }\n");
    fixture.write(
        "master/Cargo.toml",
        "[package]\nname = \"archive\"\nversion = \"0.1.0\"\n",
    );
    fixture.write("master/src/lib.rs", "pub struct PrivateArchive;\n");
    let report = fixture.assert_rejected();
    assert!(
        report
            .violations
            .iter()
            .any(|message| message.contains("missing from reviewed source inventory"))
    );
}

#[test]
fn t_l06_workspace_patch_and_replace_cannot_redirect_public_into_master() {
    for override_section in [
        "[patch.crates-io]\narchive = { path = \"master\" }\n",
        "[replace]\n\"archive:0.1.0\" = { path = \"master\" }\n",
    ] {
        let fixture = Fixture::new();
        fixture.write(
            "Cargo.toml",
            &format!("[workspace]\nmembers = [\"public\"]\n{override_section}"),
        );
        fixture.write("public/Cargo.toml", "[package]\nname = \"public-fixture\"\nversion = \"0.1.0\"\n[dependencies]\narchive = \"0.1.0\"\n");
        fixture.write("public/src/lib.rs", "pub struct PublicRole;\n");
        fixture.write(
            "master/Cargo.toml",
            "[package]\nname = \"archive\"\nversion = \"0.1.0\"\n",
        );
        fixture.write("master/src/lib.rs", "pub struct PrivateArchive;\n");
        let report = fixture.assert_rejected();
        assert!(
            report
                .violations
                .iter()
                .any(|message| message.contains("private master dependency"))
        );
    }
}

#[test]
fn t_l06_missing_target_and_inherited_dependencies_fail_closed() {
    for dependency in [
        "[target.'cfg(unix)'.build-dependencies]\nmissing = { path = \"components/missing\" }\n",
        "[dev-dependencies]\nmissing = { workspace = true }\n",
    ] {
        let fixture = Fixture::new();
        fixture.write(
            "public/Cargo.toml",
            &format!("[package]\nname = \"public-fixture\"\nversion = \"0.1.0\"\n{dependency}"),
        );
        fixture.write("public/src/lib.rs", "pub struct PublicRole;\n");
        let report = fixture.assert_rejected();
        assert!(
            report
                .violations
                .iter()
                .any(|message| message.contains("invalid role dependency graph"))
        );
    }
}
