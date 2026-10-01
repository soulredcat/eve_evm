// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::signing::{
    DurableSigner,
    types::{Hrs, SignedRecord},
};
use anyhow::{Result, ensure};

pub(in crate::consensus::signing) fn check_hrs(
    signer: &DurableSigner,
    requested: Hrs,
) -> Result<Option<&SignedRecord>> {
    ensure!(
        !signer.fenced,
        "signer fenced; reopen and reconcile required"
    );
    if let Some(record) = &signer.last {
        ensure!(
            requested >= record.hrs,
            "signer height/round/step regression"
        );
        if requested == record.hrs {
            return Ok(Some(record));
        }
    }
    Ok(None)
}
