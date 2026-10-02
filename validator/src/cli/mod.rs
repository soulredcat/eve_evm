// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod run_cli;
mod summarize_cli_failure;
mod types;

pub use run_cli::run_cli;
pub use summarize_cli_failure::summarize_cli_failure;

#[cfg(test)]
mod tests;
