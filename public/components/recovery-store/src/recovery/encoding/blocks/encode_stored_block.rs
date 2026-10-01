// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};

use crate::recovery::encoding::identity::encode_store_identity::encode_store_identity;
use crate::recovery::types::{RECORD_HEADER_BYTES, StoredBlockInput};

pub fn encode_stored_block(input: &StoredBlockInput, maximum: usize) -> Result<Vec<u8>> {
    let size = RECORD_HEADER_BYTES
        .checked_add(input.payload.len())
        .context("record size overflow")?;
    ensure!(size <= maximum, "record exceeds configured byte limit");
    let mut bytes = encode_store_identity(&input.identity);
    bytes.reserve(size - bytes.len());
    bytes.extend_from_slice(&input.parent_height.to_be_bytes());
    bytes.extend_from_slice(&input.height.to_be_bytes());
    bytes.extend_from_slice(&input.parent_block_hash);
    bytes.extend_from_slice(&input.block_hash);
    bytes.extend_from_slice(&u64::try_from(input.payload.len())?.to_be_bytes());
    bytes.extend_from_slice(&input.payload);
    Ok(bytes)
}
