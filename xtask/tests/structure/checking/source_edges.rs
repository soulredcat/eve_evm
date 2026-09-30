use crate::support::Fixture;

#[test]
fn t_l06_explicit_cargo_targets_cannot_compile_private_master_source() {
    for target in [
        "[lib]\npath = \"../master/src/lib.rs\"\n",
        "[[bin]]\nname = \"public-role\"\npath = \"../master/src/lib.rs\"\n",
        "[package]\nname = \"public-fixture\"\nversion = \"0.1.0\"\nbuild = \"../master/src/lib.rs\"\n",
    ] {
        let fixture = Fixture::new();
        fixture.write("public/Cargo.toml", target);
        fixture.write("master/src/lib.rs", "pub struct PrivateArchive;\n");
        let report = fixture.assert_rejected();
        assert!(
            report
                .violations
                .iter()
                .any(|message| message.contains("Source target escapes its owning package/role"))
        );
    }
}

#[test]
fn t_l06_path_and_conditional_path_cannot_import_private_master_source() {
    for attribute in [
        "#[path = \"../../master/src/archive.rs\"]",
        "#[cfg_attr(feature = \"private\", path = \"../../master/src/archive.rs\")]",
    ] {
        let fixture = Fixture::new();
        fixture.write("public/src/lib.rs", &format!("{attribute}\nmod archive;\n"));
        fixture.write("master/src/archive.rs", "pub struct PrivateArchive;\n");
        let report = fixture.assert_rejected();
        assert!(
            report
                .violations
                .iter()
                .any(|message| message.contains("Source target escapes its owning package/role"))
        );
    }
}

#[test]
fn t_l06_ignored_module_source_and_disguised_rust_category_are_rejected() {
    for (ignore, declaration, leaf) in [
        (
            "/public/src/hidden.rs\n",
            "mod hidden;\n",
            "public/src/hidden.rs",
        ),
        (
            "",
            "#[path = \"hidden.txt\"]\nmod hidden;\n",
            "public/src/hidden.txt",
        ),
    ] {
        let fixture = Fixture::new();
        fixture.write(".gitignore", ignore);
        fixture.write("public/src/lib.rs", declaration);
        fixture.write(leaf, "pub struct HiddenSource;\n");
        let report = fixture.assert_rejected();
        assert!(
            report
                .violations
                .iter()
                .any(|message| message.contains("invalid source edge"))
        );
    }
}

#[test]
fn t_l06_owned_explicit_and_nested_module_sources_remain_valid() {
    let fixture = Fixture::new();
    fixture.write(
        "public/src/lib.rs",
        "#[path = \"encoding/height.rs\"]\nmod height;\nmod wire { mod fields; }\n",
    );
    fixture.write(
        "public/src/encoding/height.rs",
        "pub struct Height(pub u64);\n",
    );
    fixture.write("public/src/wire/fields.rs", "pub struct Field;\n");
    fixture.assert_pass();
}

#[test]
fn t_l06_conditional_path_also_checks_the_default_module_source() {
    let fixture = Fixture::new();
    fixture.write(".gitignore", "/public/src/hidden.rs\n");
    fixture.write(
        "public/src/lib.rs",
        "#[cfg_attr(feature = \"safe\", path = \"reviewed.rs\")]\nmod hidden;\n",
    );
    fixture.write("public/src/reviewed.rs", "pub struct Reviewed;\n");
    fixture.write("public/src/hidden.rs", "pub struct Hidden;\n");
    let report = fixture.assert_rejected();
    assert!(
        report
            .violations
            .iter()
            .any(|message| message.contains("missing from reviewed inventory"))
    );
}

#[test]
fn t_l05_production_source_cannot_borrow_the_test_category() {
    let fixture = Fixture::new();
    fixture.write(
        "public/src/lib.rs",
        "#[path = \"../tests/fixture.rs\"]\nmod fixture;\n",
    );
    fixture.write(
        "public/tests/fixture.rs",
        "pub fn first_operation() {}\npub fn second_operation() {}\n",
    );
    let report = fixture.assert_rejected();
    assert!(
        report
            .violations
            .iter()
            .any(|message| message
                .contains("Production source cannot import a test-category target"))
    );
}

#[test]
fn t_l05_explicit_test_only_module_can_import_its_test_fixture() {
    let fixture = Fixture::new();
    fixture.write(
        "public/src/lib.rs",
        "#[cfg(test)]\n#[path = \"../tests/fixture.rs\"]\nmod fixture;\n",
    );
    fixture.write(
        "public/tests/fixture.rs",
        "fn setup() {}\n#[test]\nfn unit_case() {}\n",
    );
    fixture.assert_pass();
}
