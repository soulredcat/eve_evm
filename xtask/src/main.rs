// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

fn main() {
    if let Err(error) = xtask::execution::cli::run_cli::run_cli() {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}
