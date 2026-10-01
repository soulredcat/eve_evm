// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::Result;
use std::sync::Mutex;

/// A secondary shutdown error cannot replace the first actor failure.
pub(in crate::consensus::runtime) fn capture_first_channel_failure(
    slot: &Mutex<Option<anyhow::Error>>,
    error: anyhow::Error,
) -> Result<()> {
    let mut failure = slot
        .lock()
        .map_err(|_| anyhow::anyhow!("channel failure lock poisoned"))?;
    if failure.is_none() {
        *failure = Some(error);
    }
    Ok(())
}
