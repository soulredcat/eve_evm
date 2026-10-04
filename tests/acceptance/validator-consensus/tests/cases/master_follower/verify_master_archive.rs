// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::MasterFollower;
use anyhow::{Context, Result, ensure};
use serde_json::Value;

pub(super) fn verify_master_archive(
    follower: &MasterFollower,
    status: &Value,
    height: u64,
) -> Result<()> {
    ensure!(
        follower.child.is_none() && (1..=128).contains(&height),
        "MASTER_FOLLOWER_ARCHIVE_INSPECTION_OWNERSHIP"
    );
    let mut domains = 0;
    for entry in std::fs::read_dir(&follower.data)? {
        let entry = entry?;
        ensure!(
            entry.file_type()?.is_dir()
                && matches!(entry.file_name().to_str(), Some("state" | "proofs")),
            "MASTER_FOLLOWER_UNEXPECTED_VOTING_OR_PRODUCER_DOMAIN"
        );
        domains += 1;
        ensure!(domains <= 2, "MASTER_FOLLOWER_NAMESPACE_BOUND");
    }
    ensure!(domains == 2, "MASTER_FOLLOWER_NAMESPACE_MISSING");
    let mut proof_count = 0_u64;
    let mut archive_bytes = 0_u64;
    for entry in std::fs::read_dir(follower.data.join("proofs"))? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_str().context("MASTER_FOLLOWER_PROOF_NAME")?;
        ensure!(
            name.len() == 26
                && name.ends_with(".proof")
                && name.as_bytes()[..20].iter().all(u8::is_ascii_digit),
            "MASTER_FOLLOWER_UNEXPECTED_PROOF_ENTRY"
        );
        let retained_height: u64 = name[..20].parse()?;
        ensure!(
            (1..=height).contains(&retained_height)
                && name == format!("{retained_height:020}.proof"),
            "MASTER_FOLLOWER_PROOF_HEIGHT"
        );
        let metadata = std::fs::symlink_metadata(entry.path())?;
        ensure!(
            metadata.is_file() && !metadata.file_type().is_symlink() && metadata.len() > 0,
            "MASTER_FOLLOWER_PROOF_FILE_TYPE"
        );
        archive_bytes = archive_bytes
            .checked_add(metadata.len())
            .context("MASTER_FOLLOWER_ARCHIVE_ARITHMETIC")?;
        proof_count += 1;
        ensure!(proof_count <= height, "MASTER_FOLLOWER_PROOF_COUNT_BOUND");
    }
    ensure!(
        proof_count == height && status["retained_archive_bytes"] == archive_bytes,
        "MASTER_FOLLOWER_ACTUAL_ARCHIVE_STATUS"
    );
    Ok(())
}
