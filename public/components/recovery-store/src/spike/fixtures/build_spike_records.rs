// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result};

use crate::recovery::types::{DurableRecordCursor, StoredBlockInput};

/// Disposable opaque records, not signed blocks or execution validity proofs.
pub fn build_spike_records(
    cursor: DurableRecordCursor,
    count: usize,
    payload_bytes: usize,
) -> Result<Vec<StoredBlockInput>> {
    let mut records = Vec::with_capacity(count);
    let mut previous = cursor;
    for _ in 0..count {
        let height = previous
            .height
            .checked_add(1)
            .context("fixture height overflow")?;
        let mut block_hash = [0x55; 32];
        block_hash[..8].copy_from_slice(&height.to_be_bytes());
        records.push(StoredBlockInput {
            identity: cursor.identity,
            parent_height: previous.height,
            height,
            parent_block_hash: previous.block_hash,
            block_hash,
            payload: vec![0x66; payload_bytes],
        });
        previous.height = height;
        previous.block_hash = block_hash;
    }
    Ok(records)
}
