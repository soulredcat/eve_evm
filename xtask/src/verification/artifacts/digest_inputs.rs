// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::structure::{
    discovery::discover_sources::discover_sources,
    inspection::compute_source_digest::compute_source_digest,
};
use anyhow::Result;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, path::Path};

pub fn digest_inputs(root: &Path) -> Result<(String, BTreeMap<String, String>)> {
    let mut aggregate = Sha256::new();
    let mut inputs = BTreeMap::new();
    for path in discover_sources(root)? {
        let digest = compute_source_digest(root, &path)?;
        aggregate.update(path.as_bytes());
        aggregate.update([0]);
        aggregate.update(digest.as_bytes());
        aggregate.update([0]);
        inputs.insert(path, digest);
    }
    let value = aggregate
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    Ok((value, inputs))
}
