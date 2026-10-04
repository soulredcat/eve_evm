// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::structure::{
    inspection::count_physical_lines::count_physical_lines,
    types::adapter_fragment_types::AdapterPolicyFragment,
};
use anyhow::{Context, Result, ensure};
use std::{fs::File, io::Read, path::Path};

/// Bound bytes before strict TOML allocation. Normal per-file size policy still
/// reviews 201..600 lines; the hard 600-line limit can never be bypassed here.
pub(super) fn read_adapter_policy_fragment(path: &Path) -> Result<AdapterPolicyFragment> {
    const MAXIMUM_BYTES: u64 = 262_144;
    let file = File::open(path)
        .with_context(|| format!("Open adapter policy fragment {}", path.display()))?;
    let metadata = file.metadata()?;
    ensure!(
        metadata.is_file(),
        "Adapter policy fragment must remain a regular file"
    );
    ensure!(
        metadata.len() <= MAXIMUM_BYTES,
        "Adapter policy fragment byte limit"
    );
    let mut encoded = String::new();
    file.take(MAXIMUM_BYTES + 1)
        .read_to_string(&mut encoded)
        .context("Read bounded UTF-8 adapter policy fragment")?;
    ensure!(
        encoded.len() as u64 <= MAXIMUM_BYTES,
        "Adapter policy fragment byte limit"
    );
    ensure!(
        count_physical_lines(&encoded) <= 600,
        "Adapter policy fragment exceeds hard 600-line limit"
    );
    let fragment: AdapterPolicyFragment =
        toml::from_str(&encoded).context("Adapter policy fragment schema is invalid")?;
    ensure!(
        fragment.version == 1,
        "Unsupported adapter policy fragment version"
    );
    Ok(fragment)
}
