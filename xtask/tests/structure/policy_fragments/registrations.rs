// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{ADAPTER, FRAGMENT, registration, setup};

#[test]
fn flattened_fragment_and_root_duplicates_and_stale_entries_keep_existing_rejection() {
    let duplicate = setup();
    duplicate.policy(&format!(
        "adapter_files = [\"{FRAGMENT}\"]\n{}",
        registration(ADAPTER)
    ));
    assert!(
        duplicate
            .assert_rejected()
            .violations
            .iter()
            .any(|violation| violation.contains("Duplicate policy registration"))
    );
    let stale = setup();
    stale.write(
        FRAGMENT,
        &format!(
            "version = 1\n{}",
            registration("validator/src/missing_adapter.rs")
        ),
    );
    assert!(
        stale
            .assert_rejected()
            .violations
            .iter()
            .any(|violation| violation.contains("Stale policy registration"))
    );
    let incomplete = setup();
    incomplete.write(FRAGMENT, "version = 1\n[[adapters]]\npath = \"validator/src/codec_adapter.rs\"\nexternal_trait = \"\"\nreason = \"\"\nreviewer = \"\"\n");
    assert!(
        incomplete
            .assert_rejected()
            .violations
            .iter()
            .any(|violation| violation.contains("Incomplete adapter review"))
    );
}

#[test]
fn fragment_adapter_cannot_hide_operations_wrong_traits_or_source_digest_exemptions() {
    for source in [
        "impl upstream::Codec for Height { fn encode(&self) { let value = costly::calculate(); encoding::encode(value); } }\n",
        "impl upstream::Other for Height { fn encode(&self) { encoding::encode(self); } }\n",
        "fn encode() {} fn decode() {}\n",
    ] {
        let fixture = setup();
        fixture.write(ADAPTER, source);
        fixture.assert_rejected();
    }
    let fixture = setup();
    fixture.write(FRAGMENT, "version = 1\n[[adapters]]\npath = \"validator/src/codec_adapter.rs\"\nexternal_trait = \"upstream::Codec\"\nreason = \"Delegation only.\"\nreviewer = \"test-reviewer\"\nsha256 = \"00\"\n");
    super::support::assert_load_rejected(&fixture);
}
