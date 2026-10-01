// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{Fixture, REDCAT};

#[test]
fn rejects_missing_license_text() {
    let fixture = Fixture::new();
    std::fs::remove_file(
        fixture
            .root()
            .join("LICENSES/LicenseRef-Redcat-Permission-Only.txt"),
    )
    .unwrap();
    fixture.reject("missing or empty license text");
}

#[test]
fn rejects_empty_license_text() {
    let fixture = Fixture::new();
    fixture.write("LICENSES/LicenseRef-Redcat-Permission-Only.txt", " \n");
    fixture.reject("missing or empty license text");
}

#[test]
fn rejects_unused_or_nested_license_text() {
    let fixture = Fixture::new();
    fixture.write("LICENSES/nested/unused.txt", "Unused license\n");
    fixture.reject("unexpected, nested or unused license text");
}

#[test]
fn rejects_missing_preserved_law_and_platform_rights() {
    let fixture = Fixture::new();
    fixture.write(
        "LICENSE",
        "Copyright (c) 2026 Redcat. Use requires prior written permission from Redcat.\n",
    );
    fixture.reject("rights-preservation clauses");
}

#[test]
fn accepts_generated_lock_selection_without_changing_dependency_bytes() {
    let fixture = Fixture::new();
    let payload = "# Generated dependency identities.\nversion = 4\n";
    fixture.write("Cargo.lock", payload);
    fixture.annotation(
        "Cargo.lock",
        REDCAT,
        "2026 Redcat (project-specific dependency selection only)",
    );
    assert_eq!(fixture.pass().coverage.annotated_first_party, 1);
    assert_eq!(
        std::fs::read_to_string(fixture.root().join("Cargo.lock")).unwrap(),
        payload
    );
}

#[test]
fn rejects_non_text_license_payload() {
    let fixture = Fixture::new();
    std::fs::write(
        fixture
            .root()
            .join("LICENSES/LicenseRef-Redcat-Permission-Only.txt"),
        [0xff, 0xfe, 0x00],
    )
    .unwrap();
    fixture.reject("license text must be UTF-8 plain text");
}
