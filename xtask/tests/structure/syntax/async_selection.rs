// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::Fixture;

#[test]
fn bounded_tokio_selection_visits_branch_futures_guards_and_bodies() {
    for selection in [
        "result = work() => consume(result), _ = shutdown() => finish(),",
        "biased; Some(value) = work(), if enabled => { consume(value); } else => { finish(); }",
        "_ = async { work().await } => { finish(); } _ = shutdown() => (),",
    ] {
        let fixture = Fixture::new();
        fixture.write(
            "public/src/run_selection.rs",
            &format!("async fn run_selection() {{ tokio::select! {{ {selection} }} }}\n"),
        );
        let report = fixture.assert_pass();
        assert_eq!(
            report
                .files
                .iter()
                .find(|file| file.path.ends_with("run_selection.rs"))
                .unwrap()
                .operations,
            ["run_selection"]
        );
    }
}

#[test]
fn async_selection_cannot_hide_nested_operations_or_executable_initializers() {
    for source in [
        "async fn run_selection() { tokio::select! { _ = async { fn concealed() {} work().await } => (), } }",
        "async fn run_selection() { tokio::select! { _ = work() => { fn concealed() {} finish(); }, } }",
        "async fn run_selection() { tokio::select! { _ = work() => hidden!(), } }",
        "async fn run_selection() { tokio::select! { hidden!() = work() => (), } }",
        "const VALUE: u32 = tokio::select! { _ = work() => 1, };",
        "async fn run_selection() { tokio::select! { _ = work() => { let _ = |input| { if input { first(); } else { second(); } for value in values { consume(value); } }; }, } }",
    ] {
        let fixture = Fixture::new();
        fixture.write("public/src/run_selection.rs", source);
        fixture.assert_rejected();
    }
}

#[test]
fn malformed_oversized_or_aliased_async_selection_fails_closed() {
    for selection in [
        "tokio::select! {}",
        "tokio::select! { marker; _ = work() => (), }",
        "tokio::select! { _ = work(), when ready => (), }",
        "tokio::select! { else => (), _ = work() => (), }",
        "tokio::select! { _ = work() => () _ = shutdown() => () }",
        "other::select! { _ = work() => (), }",
        "select! { _ = work() => (), }",
    ]
    .into_iter()
    .map(str::to_owned)
    .chain(std::iter::once(format!(
        "tokio::select! {{ {} }}",
        "_ = work() => (),".repeat(33)
    ))) {
        let fixture = Fixture::new();
        fixture.write(
            "public/src/run_selection.rs",
            &format!("async fn run_selection() {{ {selection} }}\n"),
        );
        fixture.assert_rejected();
    }
}
