use crate::support::Fixture;

#[test]
fn t_l05_facades_reject_named_and_unnamed_execution() {
    for source in [
        "fn encode_height() {}",
        "const HEIGHT: u64 = { let value = 1; value + 1 };",
        "static HEIGHT: u64 = { let value = 1; value + 1 };",
        "const HEIGHT: u64 = derive_height();",
        "const _: () = { fn hidden_operation() {} };",
        "include!(\"concealed.rs\");",
        "unreviewed_generator!();",
    ] {
        let fixture = Fixture::new();
        fixture.write("validator/src/lib.rs", source);
        fixture.assert_rejected();
    }
}

#[test]
fn t_l05_adapter_rejects_logic_inside_call_arguments_and_receivers() {
    for body in [
        "let value = self.height + 1; encoding::encode(value)",
        "encoding::encode({ let value = self.height + 1; value })",
        "encoding::encode(&{ let value = self.height + 1; value })",
        "encoding::encode(({ let value = self.height + 1; Holder(value) }).0)",
        "({ let value = self.height + 1; Holder(value) }).encode()",
    ] {
        let fixture = Fixture::new();
        fixture.write(
            "validator/src/codec_adapter.rs",
            &format!("impl upstream::Codec for Height {{ fn encode(&self) {{ {body} }} }}\n"),
        );
        fixture.policy("[[adapters]]\npath = \"validator/src/codec_adapter.rs\"\nexternal_trait = \"upstream::Codec\"\nreason = \"Test delegation boundary.\"\nreviewer = \"test-reviewer\"\n");
        fixture.assert_rejected();
    }
}

#[test]
fn t_l05_macros_and_long_closures_cannot_hide_additional_operations() {
    for source in [
        "fn encode_height() {} macro_rules! concealed { () => { fn decode_height() {} }; }",
        "fn encode_height() { include!(\"concealed.rs\"); }",
        "fn encode_height() { let _ = || { let a = 1; let b = 2; let c = 3; a + b + c }; }",
        "fn encode_height() { let _ = || { if ready { let a = 1; let b = 2; let c = 3; a + b + c } else { 0 } }; }",
    ] {
        let fixture = Fixture::new();
        fixture.write("validator/src/encode_height.rs", source);
        fixture.assert_rejected();
    }
}

#[test]
fn a_local_expression_closure_stays_with_its_primary_operation() {
    let fixture = Fixture::new();
    fixture.write(
        "validator/src/encode_height.rs",
        "fn encode_height() { let _ = [1, 2].map(|value| value + 1); }\n",
    );
    fixture.assert_pass();
}

#[test]
fn opaque_numbered_and_catch_all_splits_are_rejected() {
    for path in [
        "validator/src/part1.rs",
        "validator/src/helpers.rs",
        "validator/src/misc.rs",
    ] {
        let fixture = Fixture::new();
        fixture.write(path, "pub struct Height(pub u64);\n");
        fixture.assert_rejected();
    }
}
