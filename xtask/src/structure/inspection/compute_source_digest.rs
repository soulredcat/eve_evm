// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::Result;
use sha2::{Digest, Sha256};
use std::path::Path;

pub fn compute_source_digest(root: &Path, path: &str) -> Result<String> {
    let bytes = std::fs::read(super::resolve_source_path::resolve_source_path(root, path)?)?;
    Ok(Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}
