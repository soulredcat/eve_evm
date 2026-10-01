// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::process::Command;

/// An extracted upstream source archive must not inherit the enclosing EVE Git stamp.
pub(crate) fn configure_go_source_build(command: &mut Command, jobs: usize) {
    command
        .args(["build", "-mod=readonly", "-trimpath", "-buildvcs=false"])
        .arg(format!("-p={jobs}"))
        .env("GOTOOLCHAIN", "local");
}
