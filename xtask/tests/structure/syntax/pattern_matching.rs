// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Fixture;

#[test]
fn bounded_matches_inspects_real_patterns_and_pure_guards() {
    for invocation in [
        "matches!(&value, Expr::Block(_))",
        "matches!(value, Some(result) if result > 0)",
        "matches!(value, Data { field: Some(_), .. } | Other(_))",
        "matches!(value, 0..=3 | 5,)",
    ] {
        let fixture = Fixture::new();
        fixture.write(
            "validator/src/inspect_value.rs",
            &format!("fn inspect_value() {{ let _ = {invocation}; }}\n"),
        );
        fixture.assert_pass();
    }
}

#[test]
fn matches_cannot_hide_opaque_patterns_executable_guards_or_extra_tokens() {
    for invocation in [
        "matches!(value, concealed!())",
        "matches!(value, const { fn concealed() {} 1 })",
        "matches!(value, Some(result) if { fn concealed() {} true })",
        "matches!({ fn concealed() {} 1 }, 1)",
        "matches!(value, 1, unexpected)",
    ] {
        let fixture = Fixture::new();
        fixture.write(
            "validator/src/inspect_value.rs",
            &format!("fn inspect_value() {{ let _ = {invocation}; }}\n"),
        );
        fixture.assert_rejected();
    }
}
