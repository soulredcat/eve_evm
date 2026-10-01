// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

fn main() {
    if eve_validator::cli::run_cli().is_err() {
        eprintln!("EVE_DEVELOPMENT_VALIDATOR_FAILED: inspect task-local sanitized diagnostics");
        std::process::exit(1);
    }
}
