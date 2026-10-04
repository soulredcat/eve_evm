// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{STAGING, open_proof_file::open_proof_file};
use anyhow::Result;
use std::{fs::File, io::Write};

/// Create one exclusive staging file and sync its exact body. Publication is separate.
pub(in crate::sync) fn write_staged_proof(directory: &File, bytes: &[u8]) -> Result<()> {
    let mut file = open_proof_file(directory, STAGING, true)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    directory.sync_all()?;
    Ok(())
}
