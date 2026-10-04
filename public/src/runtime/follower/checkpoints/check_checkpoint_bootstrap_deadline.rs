// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::time::Instant;

pub(in crate::runtime::follower) fn check_checkpoint_bootstrap_deadline(
    deadline: Instant,
) -> Result<()> {
    ensure!(
        Instant::now() < deadline,
        "PUBLIC_CHECKPOINT_BOOTSTRAP_DEADLINE"
    );
    Ok(())
}
