// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::time::Instant;

pub(super) fn ensure_submission_deadline(deadline: Instant) -> Result<()> {
    ensure!(Instant::now() < deadline, "B3_SUBMIT_PROGRESS_DEADLINE");
    Ok(())
}
