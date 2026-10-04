// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::MasterFollower;
use anyhow::{Result, ensure};
use std::{fs::File, io::Read};

/// Small real-transaction fixture bound only; this is not a production proof cap.
pub(super) fn read_master_proof(follower: &MasterFollower, height: u64) -> Result<Vec<u8>> {
    ensure!(
        follower.child.is_none() && (1..=128).contains(&height),
        "MASTER_FOLLOWER_PROOF_INSPECTION_OWNERSHIP"
    );
    let path = follower
        .data
        .join("proofs")
        .join(format!("{height:020}.proof"));
    let metadata = std::fs::symlink_metadata(&path)?;
    ensure!(
        metadata.is_file()
            && !metadata.file_type().is_symlink()
            && metadata.len() > 0
            && metadata.len() <= 1_048_576,
        "MASTER_FOLLOWER_PROOF_FIXTURE_BOUND"
    );
    let mut bytes = Vec::new();
    File::open(path)?.take(1_048_577).read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() as u64 == metadata.len(),
        "MASTER_FOLLOWER_PROOF_CHANGED_DURING_INSPECTION"
    );
    Ok(bytes)
}
