// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::metadata_types::PublicNodeMetadata;
use anyhow::{Result, ensure};
use eve_node_policy::ZoneId;
use eve_state::StateIdentity;
use std::{io::Write, path::Path};
/// Persist routing metadata beside local recovery; it does not authorize finality or shards.
pub(crate) fn persist_node_metadata(
    path: &Path,
    zone: ZoneId,
    identity: &StateIdentity,
) -> Result<()> {
    let destination = path.join("public-node-metadata.json");
    if destination.exists() {
        let attributes = std::fs::symlink_metadata(&destination)?;
        ensure!(
            attributes.is_file()
                && !attributes.file_type().is_symlink()
                && attributes.len() <= 4096,
            "public metadata must be bounded regular file"
        );
        let bytes = std::fs::read(&destination)?;
        ensure!(
            bytes.len() <= 4096,
            "public metadata byte capacity exceeded"
        );
        let metadata: serde_json::Value = serde_json::from_slice(&bytes)?;
        ensure!(
            metadata["schema"] == "EVE_PUBLIC_METADATA01"
                && metadata["genesis"] == serde_json::to_value(identity.genesis.0)?,
            "public namespace metadata genesis/schema mismatch"
        );
    }
    let value = PublicNodeMetadata {
        schema: "EVE_PUBLIC_METADATA01",
        zone_id: zone.0,
        genesis: identity.genesis.0,
        chain_id: identity.evm_chain_id,
        network_name: &identity.network_name,
        verification_mode: "LOCAL_DEV_UNAUTHENTICATED",
        authenticated_finality: false,
    };
    let temporary = path.join(format!(
        "public-node-metadata.{}.pending",
        std::process::id()
    ));
    let mut file = std::fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(&serde_json::to_vec(&value)?)?;
    file.sync_all()?;
    std::fs::rename(&temporary, &destination)?;
    #[cfg(unix)]
    std::fs::File::open(path)?.sync_all()?;
    Ok(())
}
