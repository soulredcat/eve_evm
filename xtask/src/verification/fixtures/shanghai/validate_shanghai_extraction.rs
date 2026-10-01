// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::EXTRACTED_BYTES;
use crate::provisioning::{compute_artifact_digest, resolve_contained_path};
use anyhow::{Result, ensure};
use std::{collections::BTreeMap, path::Path};

/// Verify each required regular file and exact aggregate bytes before exposing it.
pub(super) fn validate_shanghai_extraction(
    directory: &Path,
    members: &BTreeMap<String, String>,
) -> Result<()> {
    let mut bytes = 0_u64;
    for (member, digest) in members {
        let path = resolve_contained_path(directory, Path::new(member))?;
        let metadata = std::fs::symlink_metadata(&path)?;
        ensure!(
            metadata.is_file() && !metadata.file_type().is_symlink(),
            "corpus member is not a regular file"
        );
        bytes = bytes
            .checked_add(metadata.len())
            .ok_or_else(|| anyhow::anyhow!("corpus byte count overflow"))?;
        ensure!(
            bytes <= EXTRACTED_BYTES && compute_artifact_digest(&path)? == *digest,
            "corpus member size/digest mismatch"
        );
    }
    ensure!(
        bytes == EXTRACTED_BYTES,
        "incomplete extracted corpus byte identity"
    );
    Ok(())
}
