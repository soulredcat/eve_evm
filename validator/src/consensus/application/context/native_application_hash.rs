// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};
use eve_state::StateVersion;

pub(in crate::consensus::application) fn native_application_hash(
    version: &StateVersion,
) -> Result<Vec<u8>> {
    if version.height == 0 {
        ensure!(
            version.application.is_none(),
            "height-zero application commitment forbidden"
        );
        return Ok(version.content_digest.to_vec());
    }
    Ok(version
        .application
        .as_ref()
        .context("positive-height application commitment missing")?
        .0
        .0
        .to_vec())
}
