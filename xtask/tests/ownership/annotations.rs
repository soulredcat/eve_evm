// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{Fixture, REDCAT, annotation};

#[test]
fn rejects_overlapping_exact_annotations() {
    let fixture = Fixture::new();
    fixture.write("config.json", "{}\n");
    let entry = annotation("config.json", REDCAT, "2026 Redcat");
    fixture.reuse(&(entry.clone() + &entry));
    fixture.reject("overlapping ownership annotations");
}

#[test]
fn rejects_glob_and_non_normal_annotation_paths() {
    for path in [
        "*.json",
        "public/**",
        "../config.json",
        "/config.json",
        "./config.json",
        "public//config.json",
        "C:/config.json",
        "public\\config.json",
        "public/[x].json",
    ] {
        let fixture = Fixture::new();
        fixture.annotation(path, REDCAT, "2026 Redcat");
        fixture.reject("exact normal path");
    }
}

#[test]
fn rejects_stale_annotation_paths() {
    let fixture = Fixture::new();
    fixture.annotation("missing.json", REDCAT, "2026 Redcat");
    fixture.reject("stale or unsafe ownership annotation");
}

#[test]
fn rejects_override_precedence() {
    let fixture = Fixture::new();
    fixture.write("config.json", "{}\n");
    fixture.reuse(
        &annotation("config.json", REDCAT, "2026 Redcat").replace("\"closest\"", "\"override\""),
    );
    fixture.reject("closest precedence");
}

#[test]
fn rejects_changed_first_party_license() {
    let fixture = Fixture::new();
    fixture.write("config.json", "{}\n");
    fixture.write("LICENSES/MIT.txt", "Upstream fixture terms\n");
    fixture.annotation("config.json", "MIT", "2026 Redcat");
    fixture.reject("contradictory first-party annotation");
}

#[test]
fn rejects_missing_first_party_copyright() {
    let fixture = Fixture::new();
    fixture.write("config.json", "{}\n");
    fixture.annotation("config.json", REDCAT, "NOASSERTION");
    fixture.reject("contradictory first-party annotation");
}

#[test]
fn rejects_annotation_used_to_hide_commentable_source() {
    let fixture = Fixture::new();
    fixture.write("source.rs", "pub fn execute() {}\n");
    fixture.annotation("source.rs", REDCAT, "2026 Redcat");
    fixture.reject("requires its inline notice");
}

#[cfg(unix)]
#[test]
fn rejects_symlink_escape_in_annotated_source() {
    let fixture = Fixture::new();
    let outside = tempfile::NamedTempFile::new().unwrap();
    std::os::unix::fs::symlink(outside.path(), fixture.root().join("config.json")).unwrap();
    fixture.annotation("config.json", REDCAT, "2026 Redcat");
    fixture.reject("stale or unsafe ownership annotation");
}
