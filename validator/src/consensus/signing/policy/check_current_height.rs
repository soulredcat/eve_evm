// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::signing::{DurableSigner, validate_signer_config};
use anyhow::{Context, Result, ensure};
use eve_storage::state::read_state_service;

pub(in crate::consensus) fn check_current_height(
    signer: &DurableSigner,
    requested: i64,
) -> Result<()> {
    validate_signer_config(&signer.config, &signer.key, &signer.service)?;
    let view = read_state_service(&signer.service)?;
    let next = view
        .commit()
        .target
        .height
        .checked_add(1)
        .context("signer height overflow")?;
    ensure!(
        i64::try_from(next).ok() == Some(requested),
        "new signature not at current execution height"
    );
    Ok(())
}
