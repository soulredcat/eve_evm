// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_history_row::decode_history_row, read_history_bytes::read_history_bytes};
use crate::state::{
    encoding::encode_root_record,
    history::{HistorySnapshot, RetainedBlockProjection, keys::VERSION_PREFIX},
    types::{HEADER_PREFIX, ID_PREFIX, RECEIPT_PREFIX, ROOT_PREFIX, TX_PREFIX},
};
use alloy_primitives::B256;
use anyhow::{Result, anyhow, ensure};
use eve_state::{BlockPayload, decode_state_version, validate_retained_block};
pub fn read_history_block(
    snapshot: &HistorySnapshot<'_>,
    height: u64,
) -> Result<Option<RetainedBlockProjection>> {
    if height > snapshot.head.height {
        return Ok(None);
    }
    let mut used = 0_usize;
    let encoded = read_history_bytes(snapshot, VERSION_PREFIX, height, &mut used)?;
    let version =
        decode_state_version(&encoded).map_err(|e| anyhow!("corrupt history version: {e:?}"))?;
    let parent = if height > 0 {
        Some(
            decode_state_version(&read_history_bytes(
                snapshot,
                VERSION_PREFIX,
                height - 1,
                &mut used,
            )?)
            .map_err(|e| anyhow!("corrupt parent: {e:?}"))?,
        )
    } else {
        None
    };
    ensure!(
        version.height == height && version.identity == snapshot.head.identity,
        "history namespace mismatch"
    );
    let block = BlockPayload {
        header: decode_history_row(snapshot, HEADER_PREFIX, height, &mut used)?,
        transactions: decode_history_row(snapshot, TX_PREFIX, height, &mut used)?,
        receipts: decode_history_row(snapshot, RECEIPT_PREFIX, height, &mut used)?,
    };
    validate_retained_block(
        &version,
        parent.as_ref(),
        &block,
        &snapshot.storage_budget.logical,
    )
    .map_err(|e| anyhow!("invalid retained block: {e:?}"))?;
    let roots = read_history_bytes(snapshot, ROOT_PREFIX, height, &mut used)?;
    ensure!(
        roots == encode_root_record(&version),
        "corrupt retained roots"
    );
    let identity = read_history_bytes(snapshot, ID_PREFIX, height, &mut used)?;
    ensure!(identity.len() == 32, "malformed retained identity");
    Ok(Some(RetainedBlockProjection {
        version,
        commit_identity: B256::from_slice(&identity),
        block,
    }))
}
