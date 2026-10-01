// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use eve_protocol_config::genesis::{DevelopmentGenesis, input::decode_development_spec};
use std::{fs::File, io::Read, path::Path};

pub(in crate::consensus::runtime) fn load_node_public_spec(
    path: &Path,
) -> Result<(DevelopmentGenesis, serde_json::Value)> {
    let metadata = std::fs::symlink_metadata(path)?;
    ensure!(
        metadata.is_file() && metadata.len() <= 1_048_576,
        "invalid bounded public node genesis"
    );
    let mut bytes = Vec::new();
    File::open(path)?.take(1_048_577).read_to_end(&mut bytes)?;
    ensure!(
        bytes.len() <= 1_048_576,
        "public node genesis exceeded read bound"
    );
    let spec = decode_development_spec(&bytes)
        .map_err(|_| anyhow::anyhow!("invalid classical public development spec"))?;
    Ok((spec, serde_json::from_slice(&bytes)?))
}
