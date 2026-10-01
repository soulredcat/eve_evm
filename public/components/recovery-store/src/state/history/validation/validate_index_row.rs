// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::{
    encoding::height_key,
    history::keys::*,
    types::{HEADER_PREFIX, TX_PREFIX},
};
use alloy_primitives::{Bytes, keccak256};
use alloy_rlp::Decodable;
use anyhow::{Context, Result, ensure};
use eve_state::{Header, decode_state_version};
use rocksdb::{DB, SnapshotWithThreadMode};
pub(crate) fn validate_index_row(
    snapshot: &SnapshotWithThreadMode<'_, DB>,
    key: &[u8],
    value: &[u8],
    cursor: u64,
) -> Result<()> {
    if let Some(hash) = key.strip_prefix(BLOCK_LOOKUP) {
        ensure!(
            hash.len() == 32 && value.len() == 8,
            "malformed block lookup"
        );
        let height = u64::from_be_bytes(value.try_into()?);
        ensure!(height <= cursor, "block lookup beyond cursor");
        let bytes = snapshot
            .get(height_key(HEADER_PREFIX, height))?
            .context("lookup header missing")?;
        let mut remaining = bytes.as_slice();
        let header = Header::decode(&mut remaining)?;
        ensure!(
            remaining.is_empty()
                && header.number == height
                && header.hash_slow().as_slice() == hash,
            "conflicting block lookup"
        );
    } else if let Some(hash) = key.strip_prefix(TX_LOOKUP) {
        ensure!(
            hash.len() == 32 && value.len() == 12,
            "malformed transaction lookup"
        );
        let height = u64::from_be_bytes(value[..8].try_into()?);
        let index = u32::from_be_bytes(value[8..].try_into()?) as usize;
        ensure!(height <= cursor, "transaction lookup beyond cursor");
        let bytes = snapshot
            .get(height_key(TX_PREFIX, height))?
            .context("lookup transactions missing")?;
        let mut remaining = bytes.as_slice();
        let txs = Vec::<Bytes>::decode(&mut remaining)?;
        let tx = txs.get(index).context("lookup transaction index invalid")?;
        ensure!(
            remaining.is_empty() && keccak256(tx).as_slice() == hash,
            "conflicting transaction lookup"
        );
    } else if let Some(encoded) = key.strip_prefix(VERSION_PREFIX) {
        ensure!(encoded.len() == 8, "malformed version lookup");
        let height = u64::from_be_bytes(encoded.try_into()?);
        let version =
            decode_state_version(value).map_err(|e| anyhow::anyhow!("invalid version: {e:?}"))?;
        ensure!(
            height <= cursor && version.height == height,
            "conflicting version lookup"
        );
    } else {
        anyhow::bail!("unexpected index row");
    }
    Ok(())
}
