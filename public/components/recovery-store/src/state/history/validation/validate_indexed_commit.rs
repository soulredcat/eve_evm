// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::encoding::decode_head_marker;
use crate::state::history::{indexing::encode_index_entries, keys::INDEX_CURSOR};
use alloy_primitives::B256;
use anyhow::{Result, ensure};
use eve_state::StateCommit;
use rocksdb::{DB, SnapshotWithThreadMode};
pub(crate) fn validate_indexed_commit(
    snapshot: &SnapshotWithThreadMode<'_, DB>,
    commit: &StateCommit,
    identity: B256,
) -> Result<()> {
    let Some(cursor) = snapshot.get(INDEX_CURSOR)? else {
        return Ok(());
    };
    let (height, _) = decode_head_marker(&cursor)?;
    if commit.target.height > height {
        return Ok(());
    }
    for (key, expected) in encode_index_entries(commit, identity)? {
        if key != INDEX_CURSOR {
            ensure!(
                snapshot.get(key)?.as_deref() == Some(expected.as_slice()),
                "corrupted or missing history entry"
            );
        }
    }
    Ok(())
}
