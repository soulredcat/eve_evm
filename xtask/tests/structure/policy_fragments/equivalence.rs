// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::support::{ADAPTER, load, registration, setup};
use crate::support::Fixture;

#[test]
fn absent_fragment_field_preserves_existing_root_policy_and_adapter_classification() {
    let root_only = Fixture::new();
    root_only.write(ADAPTER, "impl upstream::Codec for Height {\n    fn encode(&self) { encoding::encode(self); }\n    fn decode(&self) { decoding::decode(self); }\n}\n");
    root_only.policy(&registration(ADAPTER));
    let original = load(&root_only).unwrap();
    assert!(original.adapter_files.is_empty());
    let fragmented = setup();
    let loaded = load(&fragmented).unwrap();
    assert_eq!(original.version, loaded.version);
    assert_eq!(original.current_bulk, loaded.current_bulk);
    assert_eq!(original.adapters.len(), loaded.adapters.len());
    for (before, after) in original.adapters.iter().zip(&loaded.adapters) {
        assert_eq!(before.path, after.path);
        assert_eq!(before.external_trait, after.external_trait);
        assert_eq!(before.reason, after.reason);
        assert_eq!(before.reviewer, after.reviewer);
    }
    for fixture in [&root_only, &fragmented] {
        let report = fixture.assert_pass();
        let file = report
            .files
            .iter()
            .find(|file| file.path == ADAPTER)
            .unwrap();
        assert_eq!(file.kind, "adapter");
        assert!(file.violations.is_empty());
    }
}

#[test]
fn strict_fragments_merge_in_order_with_root_entries_without_other_capabilities() {
    let fixture = setup();
    fixture.write(
        "validator/src/second_adapter.rs",
        "impl upstream::Codec for Second { fn encode(&self) { encoding::encode(self); } }\n",
    );
    fixture.write(
        "config/structure/second.toml",
        &format!(
            "version = 1\n{}",
            registration("validator/src/second_adapter.rs")
        ),
    );
    fixture.policy(&format!("adapter_files = [\"config/structure/adapters.toml\", \"config/structure/second.toml\"]\n{}", registration("validator/src/root_adapter.rs")));
    fixture.write(
        "validator/src/root_adapter.rs",
        "impl upstream::Codec for Root { fn encode(&self) { encoding::encode(self); } }\n",
    );
    let loaded = load(&fixture).unwrap();
    assert_eq!(
        loaded
            .adapters
            .iter()
            .map(|entry| entry.path.as_str())
            .collect::<Vec<_>>(),
        [
            "validator/src/root_adapter.rs",
            ADAPTER,
            "validator/src/second_adapter.rs"
        ]
    );
    assert!(loaded.exclusions.is_empty());
    assert!(loaded.size_reviews.is_empty());
    assert!(loaded.exceptions.is_empty());
    fixture.assert_pass();
}
