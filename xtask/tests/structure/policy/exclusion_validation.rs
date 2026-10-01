// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Fixture;
use xtask::structure::inspection::compute_source_digest::compute_source_digest;

#[test]
fn t_l04_explicit_generated_lockfile_and_pinned_vendor_fixture_are_reported() {
    let fixture = Fixture::new();
    fixture.write(
        "Cargo.lock",
        &"# Cargo-generated lockfile fixture.\n".repeat(601),
    );
    fixture.write("tests/fixtures/upstream/vector.json", "{\"height\":258}\n");
    let digest =
        compute_source_digest(fixture.root(), "tests/fixtures/upstream/vector.json").unwrap();
    fixture.policy(&format!(
        "[[exclusions]]\npath = \"Cargo.lock\"\nkind = \"generated\"\nsource = \"https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html\"\ngenerator = \"Cargo 1.97.1 generate-lockfile\"\nreason = \"Cargo-generated dependency identities.\"\n[[exclusions]]\npath = \"tests/fixtures/upstream/vector.json\"\nkind = \"vendor\"\nsource = \"https://example.test/upstream/0123456789abcdef/vector.json\"\nsha256 = \"{digest}\"\nreason = \"Synthetic pinned upstream-fixture provenance test.\"\n"
    ));
    let report = fixture.assert_pass();
    assert_eq!(
        report.exclusions,
        ["Cargo.lock", "tests/fixtures/upstream/vector.json"]
    );
    assert!(
        !report
            .files
            .iter()
            .any(|file| report.exclusions.contains(&file.path))
    );
    fixture.write("tests/fixtures/upstream/vector.json", "{\"height\":259}\n");
    let changed = fixture.assert_rejected();
    assert!(
        changed
            .violations
            .iter()
            .any(|message| message.contains("digest mismatch"))
    );
}

#[test]
fn t_l04_generated_or_vendor_labels_cannot_hide_first_party_rust() {
    for path in [
        "public/src/encode_height.rs",
        "master/src/encode_height.rs",
        "validator/src/encode_height.rs",
        "xtask/src/encode_height.rs",
        "xtask/tests/encode_height.rs",
        "tests/encode_height.rs",
        "tools/generation/emit_fixture.rs",
        "generated/encode_height.rs",
    ] {
        for kind in ["generated", "vendor"] {
            let fixture = Fixture::new();
            fixture.write(path, "fn encode_height() {} fn decode_height() {}\n");
            let digest = compute_source_digest(fixture.root(), path).unwrap();
            fixture.policy(&format!("[[exclusions]]\npath = \"{path}\"\nkind = \"{kind}\"\nsource = \"https://example.test/first-party-source\"\ngenerator = \"tools/generation/emit_fixture.rs\"\nsha256 = \"{digest}\"\nreason = \"Attempt to conceal handwritten code.\"\n"));
            fixture.assert_rejected();
        }
    }
}

#[test]
fn t_l04_output_exclusion_does_not_exempt_its_handwritten_generator() {
    let fixture = Fixture::new();
    fixture.write(
        "tools/generation/emit_fixture.rs",
        "fn emit_fixture() {} fn choose_profile() {}\n",
    );
    fixture.write("generated/encoding.json", &"{}\n".repeat(601));
    fixture.policy("[[exclusions]]\npath = \"generated/encoding.json\"\nkind = \"generated\"\nsource = \"tools/generation/emit_fixture.rs\"\ngenerator = \"tools/generation/emit_fixture.rs\"\nreason = \"Machine-generated JSON fixture.\"\n");
    let report = fixture.assert_rejected();
    assert!(
        report
            .violations
            .iter()
            .any(|message| message.contains("emit_fixture.rs"))
    );
}

#[test]
fn exclusion_requires_identity_and_nonempty_provenance() {
    for registration in [
        "kind = \"generated\"\nsource = \"Cargo\"\nreason = \"Generated fixture\"\n",
        "kind = \"vendor\"\nsource = \"Upstream\"\nsha256 = \"abc\"\nreason = \"Vendor fixture\"\n",
        "kind = \"generated\"\nsource = \"\"\ngenerator = \"Cargo 1.97.1\"\nreason = \"Generated fixture\"\n",
        "kind = \"generated\"\nsource = \"Cargo\"\ngenerator = \"Cargo 1.97.1\"\nreason = \"\"\n",
    ] {
        let fixture = Fixture::new();
        fixture.write("Cargo.lock", "# Small dependency lock fixture.\n");
        fixture.policy(&format!(
            "[[exclusions]]\npath = \"Cargo.lock\"\n{registration}"
        ));
        fixture.assert_rejected();
    }
}
