// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::{StateSnapshot, encoding::height_key, types::COMMIT_PREFIX};
use anyhow::{Context, Result, ensure};

/// Borrow the actual retained payload without decoding or copying it. This local
/// length observation is neither whole-row validity nor finality authentication.
pub fn snapshot_commit_encoded_length(
    snapshot: &StateSnapshot<'_>,
    height: u64,
) -> Result<Option<usize>> {
    if height > snapshot.version.height {
        return Ok(None);
    }
    let bytes = snapshot
        .snapshot
        .get_pinned(height_key(COMMIT_PREFIX, height))?
        .context("missing retained snapshot recovery commit")?;
    ensure!(
        bytes.len() <= snapshot.budget.logical.maximum_commit_bytes,
        "retained snapshot commit exceeds logical byte budget"
    );
    Ok(Some(bytes.len()))
}
