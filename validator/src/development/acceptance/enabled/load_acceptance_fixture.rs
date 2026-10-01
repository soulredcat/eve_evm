// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    AcceptanceFixture, types::FixtureInput,
    validate_acceptance_fixture::validate_acceptance_fixture,
};
use anyhow::{Result, ensure};
use eve_protocol_config::genesis::DevelopmentGenesis;
use sha2::{Digest, Sha256};
use std::{fs::File, io::Read, path::Path, sync::Arc};

pub(crate) fn load_acceptance_fixture(
    path: Option<&Path>,
    genesis: &DevelopmentGenesis,
) -> Result<Option<Arc<AcceptanceFixture>>> {
    super::super::validate_acceptance_profile::validate_acceptance_profile(
        genesis,
        path.is_some(),
    )?;
    let Some(path) = path else { return Ok(None) };
    let metadata = std::fs::symlink_metadata(path)?;
    ensure!(
        metadata.is_file() && !metadata.file_type().is_symlink() && metadata.len() <= 16_384,
        "acceptance fixture must be a bounded regular public file"
    );
    let mut raw = Vec::new();
    File::open(path)?.take(16_385).read_to_end(&mut raw)?;
    ensure!(
        !raw.is_empty() && raw.len() <= 16_384,
        "acceptance fixture byte limit"
    );
    let input: FixtureInput = serde_json::from_slice(&raw)?;
    let digest = Sha256::digest(&raw).into();
    Ok(Some(Arc::new(validate_acceptance_fixture(
        input, digest, genesis,
    )?)))
}
