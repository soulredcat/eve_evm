// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ensure_submission_deadline::ensure_submission_deadline;
use anyhow::Result;
use std::time::{Duration, Instant};

pub(super) fn wait_for_submission_poll(deadline: Instant) -> Result<()> {
    ensure_submission_deadline(deadline)?;
    std::thread::sleep(
        Duration::from_millis(150).min(deadline.saturating_duration_since(Instant::now())),
    );
    ensure_submission_deadline(deadline)
}
