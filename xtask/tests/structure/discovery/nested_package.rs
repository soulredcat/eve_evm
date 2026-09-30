use crate::support::Fixture;

#[test]
fn t_l01_deep_modules_build_test_and_package_without_a_depth_ceiling() {
    let fixture = Fixture::new();
    fixture.write(
        "validator/components/encoding/Cargo.toml",
        "[package]\nname = \"nested-encoding-fixture\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    fixture.write(
        "validator/components/encoding/src/lib.rs",
        "pub mod receipts;\n",
    );
    let source = "validator/components/encoding/src/receipts";
    for (parent, child) in [
        ("", "encoding"),
        ("/encoding", "fields"),
        ("/encoding/fields", "height"),
        ("/encoding/fields/height", "encode_height"),
    ] {
        fixture.write(
            &format!("{source}{parent}/mod.rs"),
            &format!("pub mod {child};\n"),
        );
    }
    fixture.write(
        &format!("{source}/encoding/fields/height/encode_height.rs"),
        "pub fn encode_height(height: u64) -> [u8; 8] {\n    height.to_be_bytes()\n}\n",
    );
    fixture.write(
        "validator/components/encoding/tests/height_encoding.rs",
        "use nested_encoding_fixture::receipts::encoding::fields::height::encode_height::encode_height;\n\n#[test]\nfn height_uses_fixed_big_endian_bytes() {\n    assert_eq!(encode_height(258), [0, 0, 0, 0, 0, 0, 1, 2]);\n}\n",
    );
    fixture.assert_pass();
    for arguments in [
        vec!["test", "--offline"],
        vec!["package", "--offline", "--allow-dirty"],
    ] {
        let result = fixture.cargo("validator/components/encoding", &arguments);
        assert!(
            result.status.success(),
            "{arguments:?}: {}\n{}",
            String::from_utf8_lossy(&result.stdout),
            String::from_utf8_lossy(&result.stderr),
        );
    }
    assert!(
        fixture
            .root()
            .join("target/package/nested-encoding-fixture-0.1.0.crate")
            .is_file()
    );
}
