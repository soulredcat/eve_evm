// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use std::{io::Read, path::Path};

pub(in crate::development::engine) fn read_engine_file(
    path: &Path,
    maximum: u64,
) -> Result<Vec<u8>> {
    let file = std::fs::File::open(path)?;
    ensure!(
        file.metadata()?.is_file() && file.metadata()?.len() <= maximum,
        "engine metadata file byte/type limit"
    );
    let mut bytes = Vec::new();
    file.take(
        maximum
            .checked_add(1)
            .ok_or_else(|| anyhow::anyhow!("engine read limit overflow"))?,
    )
    .read_to_end(&mut bytes)?;
    ensure!(
        u64::try_from(bytes.len())? <= maximum,
        "engine metadata read byte limit"
    );
    Ok(bytes)
}
