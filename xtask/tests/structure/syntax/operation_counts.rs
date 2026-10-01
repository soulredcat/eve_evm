// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Fixture;
use xtask::structure::syntax::inspect_rust_syntax::inspect_rust_syntax;

#[test]
fn t_l02_comments_strings_and_foreign_declarations_are_not_operations() {
    let source = r#"
// fn fabricated_comment() {}
const TEXT: &str = "fn fabricated_string() {}";
unsafe extern "C" { fn upstream_declaration(); }
pub fn encode_height() { let _ = TEXT; }
"#;
    let inventory = inspect_rust_syntax(source).unwrap();
    assert_eq!(inventory.operations, ["encode_height"]);
    let fixture = Fixture::new();
    fixture.write("validator/src/encode_height.rs", source);
    fixture.assert_pass();
}

#[test]
fn t_l02_two_production_operations_fail_even_below_two_hundred_lines() {
    let fixture = Fixture::new();
    fixture.write(
        "validator/src/encode_height.rs",
        "pub fn encode_height() {}\nfn decode_height() {}\n",
    );
    let report = fixture.assert_rejected();
    let file = report
        .files
        .iter()
        .find(|file| file.path.ends_with("encode_height.rs"))
        .unwrap();
    assert_eq!(file.operations, ["encode_height", "decode_height"]);
}

#[test]
fn t_l02_associated_and_nested_operations_are_counted() {
    for source in [
        "struct Height; impl Height { fn encode_height() {} fn decode_height() {} }",
        "fn encode_height() { fn decode_height() {} decode_height(); }",
        "fn encode_height() {} mod decoding { fn decode_height() {} }",
        "trait Codec { fn encode_height() {} fn decode_height() {} }",
    ] {
        let inventory = inspect_rust_syntax(source).unwrap();
        assert_eq!(
            inventory.operations,
            ["encode_height", "decode_height"],
            "{source}"
        );
        let fixture = Fixture::new();
        fixture.write("validator/src/encode_height.rs", source);
        fixture.assert_rejected();
    }
}

#[test]
fn only_test_configuration_excludes_test_helpers_from_production_counts() {
    let source = r#"
fn encode_height() {}
#[cfg(test)] mod tests { fn setup() {} #[test] fn checks_encoding() {} }
#[cfg(test)] fn fixture() {}
#[test] fn isolated_test() {}
#[cfg(test)] trait TestSetup { fn fixture_method() { let _ = 1; } }
"#;
    assert_eq!(
        inspect_rust_syntax(source).unwrap().operations,
        ["encode_height"]
    );
    let feature_source = "fn encode_height() {} #[cfg(feature = \"test\")] fn decode_height() {}";
    assert_eq!(
        inspect_rust_syntax(feature_source).unwrap().operations,
        ["encode_height", "decode_height"],
    );
    let fixture = Fixture::new();
    fixture.write("validator/src/encode_height.rs", feature_source);
    fixture.assert_rejected();
}

#[test]
fn behavior_file_name_must_match_its_primary_operation() {
    let fixture = Fixture::new();
    fixture.write("validator/src/encoding.rs", "fn encode_height() {}\n");
    let report = fixture.assert_rejected();
    assert!(
        report
            .violations
            .iter()
            .any(|message| message.contains("named after"))
    );
}
