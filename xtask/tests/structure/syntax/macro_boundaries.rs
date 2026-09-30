use crate::support::Fixture;

#[test]
fn t_l05_unknown_expression_and_statement_macros_fail_closed() {
    for source in [
        "fn encode_height() { let _ = concealed!(); }\n",
        "fn encode_height() { concealed! { fn hidden() {} } }\n",
        "fn encode_height() { let _ = unknown::format!(\"value\"); }\n",
    ] {
        let fixture = Fixture::new();
        fixture.write("validator/src/encode_height.rs", source);
        let report = fixture.assert_rejected();
        assert!(
            report
                .violations
                .iter()
                .any(|message| message.contains("macro/closure hides"))
        );
    }
}

#[test]
fn t_l05_diagnostic_macro_arguments_cannot_hide_blocks_branches_or_functions() {
    for argument in [
        "{ fn concealed() {} 1 }",
        "if ready { 1 } else { 2 }",
        "match value { 0 => 1, _ => 2 }",
        "async { 1 }",
    ] {
        let fixture = Fixture::new();
        fixture.write(
            "validator/src/encode_height.rs",
            &format!("fn encode_height() {{ let _ = format!(\"{{}}\", {argument}); }}\n"),
        );
        fixture.assert_rejected();
    }
}

#[test]
fn t_l05_diagnostic_literals_and_pure_expression_arguments_remain_valid() {
    let fixture = Fixture::new();
    fixture.write("validator/src/encode_height.rs", "fn encode_height() { let value = 1; let _ = format!(\"fn concealed() {{}} {}\", value + 1); let _ = vec![0; 4]; let _ = matches!(value, 1 | 2); ensure!(value == 1, \"value: {}\", format!(\"{value}\")); }\n");
    fixture.assert_pass();
}

#[test]
fn t_l05_directory_segments_cannot_hide_generic_or_numbered_production_code() {
    for segment in [
        "shared", "utils", "helpers", "common", "misc", "manager", "part17",
    ] {
        let fixture = Fixture::new();
        fixture.write(
            &format!("validator/src/{segment}/encode_height.rs"),
            "fn encode_height() {}\n",
        );
        let report = fixture.assert_rejected();
        assert!(
            report
                .violations
                .iter()
                .any(|message| message.contains("catch-all or opaque split"))
        );
    }
    let fixture = Fixture::new();
    fixture.write(
        "validator/tests/common/fixture.rs",
        "fn setup() {}\n#[test]\nfn example() {}\n",
    );
    fixture.assert_pass();
}

#[test]
fn t_l05_compile_time_cfg_macros_use_only_structured_metadata_predicates() {
    let fixture = Fixture::new();
    fixture.write("validator/src/encode_height.rs", "fn encode_height() { let _ = cfg!(target_os = \"linux\"); let _ = cfg!(all(target_arch = \"x86_64\", not(feature = \"private\"))); }\n");
    fixture.assert_pass();
    for predicate in [
        "concealed()",
        "any(fn hidden() {})",
        "target_os = { derive_platform() }",
    ] {
        let fixture = Fixture::new();
        fixture.write(
            "validator/src/encode_height.rs",
            &format!("fn encode_height() {{ let _ = cfg!({predicate}); }}\n"),
        );
        fixture.assert_rejected();
    }
}
