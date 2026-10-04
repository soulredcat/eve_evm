// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{open_proof_file::open_proof_file, proof_file_name};
use anyhow::Result;
use std::fs::File;

/// A completed name surviving restart is reverified by the caller, then resynced
/// before any proof-ahead database reconciliation. Presence alone grants no ACK.
pub(in crate::sync) fn sync_completed_proof(directory: &File, height: u64) -> Result<()> {
    let file = open_proof_file(directory, &proof_file_name(height)?, false)?;
    file.sync_all()?;
    directory.sync_all()?;
    Ok(())
}
