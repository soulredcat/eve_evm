use crate::support::Fixture;

#[test]
fn t_l05_literal_struct_constants_and_pure_update_bases_remain_declarations() {
    let fixture = Fixture::new();
    fixture.write("validator/src/protocol/types.rs", "struct Inner { height: u64 }\nstruct Schedule { dispatch: u64, inner: Inner, limits: [u64; 2] }\nconst BASE: Schedule = Schedule { dispatch: 5_000, inner: Inner { height: 1 }, limits: [4, 64] };\nconst NEXT: Schedule = Schedule { dispatch: 5_001, ..BASE };\n");
    let report = fixture.assert_pass();
    let file = report
        .files
        .iter()
        .find(|file| file.path.ends_with("types.rs"))
        .unwrap();
    assert_eq!(file.kind, "declaration");
    assert!(file.operations.is_empty());
}

#[test]
fn t_l05_struct_constant_fields_and_updates_cannot_hide_executable_behavior() {
    for expression in [
        "Schedule { height: derive_height() }",
        "Schedule { height: { let height = 1; height } }",
        "Schedule { height: if true { 1 } else { 2 } }",
        "Schedule { nested: Inner { height: derive_height() } }",
        "Schedule { ..derive_schedule() }",
        "Schedule { ..{ let value = BASE; value } }",
    ] {
        let fixture = Fixture::new();
        fixture.write(
            "validator/src/protocol/types.rs",
            &format!("const SCHEDULE: Schedule = {expression};\n"),
        );
        let report = fixture.assert_rejected();
        assert!(
            report
                .violations
                .iter()
                .any(|message| message.contains("initializer hides executable behavior"))
        );
    }
}
