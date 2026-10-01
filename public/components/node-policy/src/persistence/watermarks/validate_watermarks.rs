// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::{PublicWatermarks, StateAuthentication};

pub fn validate_watermarks(value: PublicWatermarks) -> Result<(), &'static str> {
    if value.applied.0 > value.finalized.0
        || value.durable_recovery.0 > value.finalized.0
        || value.checkpoint.0 > value.durable_recovery.0
        || value.authenticated_state.0 > value.applied.0
        || value.authenticated_snapshot_height > value.checkpoint.0
        || value.authenticated_snapshot_height > value.authenticated_state.0
        || value.oldest_retained_height > value.durable_recovery.0
    {
        return Err("inconsistent public watermarks");
    }
    if let StateAuthentication::NextCertifiedHeader { header_height } = value.authentication
        && (value.authenticated_state.0.checked_add(1) != Some(header_height)
            || header_height > value.finalized.0)
    {
        return Err("wrong H/H+1 state anchor");
    }
    Ok(())
}
