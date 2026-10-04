// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::cases::public_follower::PublicFollower;
use anyhow::{Result, ensure};
use std::os::unix::fs::MetadataExt;

/// Presence assertion while the producer owns the namespace; canonical byte validation follows reaping.
pub(super) fn assert_completed_content_present(follower: &PublicFollower) -> Result<()> {
    let root = follower.data.join(".checkpoints/content");
    let metadata = std::fs::symlink_metadata(&root)?;
    ensure!(
        metadata.is_dir() && !metadata.file_type().is_symlink() && metadata.mode() & 0o077 == 0,
        "CHECKPOINT_INTERRUPTION_CONTENT_ROOT"
    );
    let mut entries = std::fs::read_dir(&root)?;
    let entry = entries
        .next()
        .transpose()?
        .ok_or_else(|| anyhow::anyhow!("CHECKPOINT_INTERRUPTION_CONTENT_MISSING"))?;
    ensure!(
        entries.next().is_none(),
        "CHECKPOINT_INTERRUPTION_CONTENT_NAMESPACE_COUNT"
    );
    let name = entry.file_name();
    let name = name
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("CHECKPOINT_INTERRUPTION_CONTENT_NAME"))?;
    ensure!(
        name.len() == 64
            && name
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        "CHECKPOINT_INTERRUPTION_CONTENT_NAME"
    );
    let namespace = std::fs::symlink_metadata(entry.path())?;
    ensure!(
        namespace.is_dir() && !namespace.file_type().is_symlink(),
        "CHECKPOINT_INTERRUPTION_CONTENT_NAMESPACE"
    );
    let completion = std::fs::symlink_metadata(entry.path().join("complete.bin"))?;
    ensure!(
        completion.is_file()
            && !completion.file_type().is_symlink()
            && completion.nlink() == 1
            && completion.len() == 80,
        "CHECKPOINT_INTERRUPTION_CONTENT_NOT_COMPLETE"
    );
    Ok(())
}
