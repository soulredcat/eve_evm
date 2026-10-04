// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "../../../tests/delta/fixtures.rs"]
mod fixtures;
mod late_materialization;
#[path = "../../../tests/delta/server.rs"]
mod server;
use std::{cell::Cell, time::Duration};
thread_local! {
    pub(super) static MATERIALIZATION_DELAY: Cell<Duration> = const { Cell::new(Duration::ZERO) };
}
pub(super) fn hold_materialization() {
    MATERIALIZATION_DELAY.with(|delay| std::thread::sleep(delay.get()));
}
