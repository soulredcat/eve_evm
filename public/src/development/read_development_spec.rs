// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use eve_protocol_config::genesis::{DevelopmentGenesis, input::decode_development_spec};
use std::{io::Read, path::Path};
pub(crate) fn read_development_spec(path: &Path) -> Result<DevelopmentGenesis> {
    let metadata = std::fs::symlink_metadata(path)?;
    ensure!(
        metadata.is_file() && !metadata.file_type().is_symlink() && metadata.len() <= 1_048_576,
        "genesis must be bounded regular public specification"
    );
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(1_048_577)
        .read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() <= 1_048_576,
        "genesis grew beyond byte capacity"
    );
    decode_development_spec(&bytes)
        .map_err(|e| anyhow::anyhow!("invalid development genesis: {e:?}"))
}
