// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};

pub(in crate::sync) fn proof_file_name(height: u64) -> Result<String> {
    ensure!(height > 0, "MASTER_PROOF_GENESIS_HAS_NO_CERTIFICATE");
    Ok(format!("{height:020}.proof"))
}
