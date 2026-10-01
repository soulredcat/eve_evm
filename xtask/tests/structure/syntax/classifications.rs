// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Fixture;

#[test]
fn t_l05_type_facade_entry_and_test_categories_are_distinct() {
    let fixture = Fixture::new();
    for (path, source) in [
        (
            "validator/src/height_types.rs",
            "pub struct Height(pub u64);\n",
        ),
        ("validator/src/lib.rs", "mod height_types;\n"),
        (
            "validator/src/main.rs",
            "fn main() { bootstrap::start(); }\n",
        ),
        (
            "validator/tests/encode_height.rs",
            "fn setup() {}\n#[test]\nfn round_trip() {}\n#[test]\nfn boundary() {}\n",
        ),
    ] {
        fixture.write(path, source);
    }
    let report = fixture.assert_pass();
    for (path, kind) in [
        ("validator/src/height_types.rs", "declaration"),
        ("validator/src/lib.rs", "facade"),
        ("validator/src/main.rs", "entry"),
        ("validator/tests/encode_height.rs", "test"),
    ] {
        assert_eq!(
            report
                .files
                .iter()
                .find(|file| file.path == path)
                .unwrap()
                .kind,
            kind
        );
    }
}

#[test]
fn t_l05_registered_external_trait_adapter_only_delegates() {
    let fixture = Fixture::new();
    fixture.write(
        "validator/src/codec_adapter.rs",
        "impl upstream::Codec for Height {\n    fn encode(&self) { encoding::encode(self); }\n    fn decode(&self) { decoding::decode(self); }\n}\n",
    );
    fixture.policy(adapter_review());
    let report = fixture.assert_pass();
    assert_eq!(
        report
            .files
            .iter()
            .find(|file| file.path.ends_with("codec_adapter.rs"))
            .unwrap()
            .kind,
        "adapter"
    );
}

#[test]
fn t_l05_adapter_registration_does_not_bless_free_or_inherent_operations() {
    for source in [
        "fn encode() {} fn decode() {}",
        "impl Height { fn encode(&self) { encoding::encode(self); } fn decode(&self) { decoding::decode(self); } }",
    ] {
        let fixture = Fixture::new();
        fixture.write("validator/src/codec_adapter.rs", source);
        fixture.policy(adapter_review());
        fixture.assert_rejected();
    }
}

#[test]
fn adapter_review_must_identify_the_trait_actually_implemented() {
    let fixture = Fixture::new();
    fixture.write(
        "validator/src/codec_adapter.rs",
        "impl upstream::UnreviewedCodec for Height { fn encode(&self) { encoding::encode(self); } }\n",
    );
    fixture.policy(adapter_review());
    fixture.assert_rejected();
}

#[test]
fn test_named_production_leaf_remains_subject_to_the_operation_rule() {
    let fixture = Fixture::new();
    fixture.write(
        "validator/src/encoding_tests.rs",
        "fn encode() {} fn decode() {}\n",
    );
    fixture.assert_rejected();
}

#[test]
fn entrypoint_rejects_additional_named_behavior() {
    let fixture = Fixture::new();
    fixture.write("validator/src/main.rs", "fn main() {} fn load_keys() {}\n");
    fixture.assert_rejected();
}

fn adapter_review() -> &'static str {
    "[[adapters]]\npath = \"validator/src/codec_adapter.rs\"\nexternal_trait = \"upstream::Codec\"\nreason = \"Cohesive codec callbacks delegate to named operations.\"\nreviewer = \"test-reviewer\"\n"
}
