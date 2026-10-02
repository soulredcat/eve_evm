// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::path::Path;

/// Share the bounded runtime/CLI diagnostic path across discovery and readiness exits.
pub(crate) fn summarize_exited_validator(data: &Path, stderr: &Path, process_id: u32) -> String {
    format!(
        "{},{}",
        super::summarize_node_failure(data, process_id),
        super::summarize_cli_failure(stderr, process_id)
    )
}
