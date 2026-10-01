// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};

/// Local diagnostic/temporary names only; wall clock never controls consensus or authorization.
/// Exclusive file creation by the caller rejects any collision without overwriting existing data.
pub(in crate::development::engine) fn engine_artifact_prefix(operation: &str) -> Result<String> {
    ensure!(
        matches!(operation, "write" | "run" | "version" | "init" | "node-id"),
        "unsupported engine artifact operation"
    );
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_nanos();
    Ok(format!("engine-{operation}-{}-{nanos}", std::process::id()))
}
