// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

fn main() {
    if let Err(error) = eve_validator::cli::run_cli() {
        eprintln!(
            "EVE_DEVELOPMENT_VALIDATOR_FAILED {}: {}",
            std::process::id(),
            eve_validator::cli::summarize_cli_failure(&error)
        );
        std::process::exit(1);
    }
}
