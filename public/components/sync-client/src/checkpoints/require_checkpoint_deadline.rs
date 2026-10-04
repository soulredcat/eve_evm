// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::time::Instant;
pub(super) fn require_checkpoint_deadline(deadline: Instant) -> Result<()> {
    ensure!(Instant::now() < deadline, "SYNC_CHECKPOINT_DEADLINE");
    Ok(())
}
