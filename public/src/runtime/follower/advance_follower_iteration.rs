// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{AppliedOwner, poll_applied_durability};
use crate::sync::follower::{ValidatorFollowerSource, follow_next_validator_delta};
use anyhow::Result;

pub(super) fn advance_follower_iteration(
    owner: &mut AppliedOwner,
    source: ValidatorFollowerSource,
) -> Result<bool> {
    poll_applied_durability(owner)
        .map_err(|error| anyhow::anyhow!("follower persistence failed: {error:?}"))?;
    // Source refusal preserves the actual prior publication and bounded leases.
    match follow_next_validator_delta(owner, source) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
