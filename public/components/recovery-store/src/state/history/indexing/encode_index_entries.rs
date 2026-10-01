// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::{
    encoding::{encode_head_marker, height_key},
    history::keys::*,
};
use alloy_primitives::{B256, keccak256};
use anyhow::{Result, anyhow};
use eve_state::{StateCommit, encode_state_version};
use std::collections::BTreeMap;

pub(crate) fn encode_index_entries(
    commit: &StateCommit,
    identity: B256,
) -> Result<BTreeMap<Vec<u8>, Vec<u8>>> {
    let mut entries = BTreeMap::new();
    entries.insert(
        [BLOCK_LOOKUP, commit.target.execution_hash.0.as_slice()].concat(),
        commit.target.height.to_be_bytes().to_vec(),
    );
    entries.insert(
        height_key(VERSION_PREFIX, commit.target.height),
        encode_state_version(&commit.target)
            .map_err(|e| anyhow!("invalid history version: {e:?}"))?
            .to_vec(),
    );
    for (index, raw) in commit.block.transactions.iter().enumerate() {
        let index = u32::try_from(index)?;
        let location = [
            commit.target.height.to_be_bytes().as_slice(),
            index.to_be_bytes().as_slice(),
        ]
        .concat();
        let hash = keccak256(raw);
        if entries
            .insert([TX_LOOKUP, hash.as_slice()].concat(), location)
            .is_some()
        {
            anyhow::bail!("duplicate transaction identity within retained block");
        }
    }
    entries.insert(
        INDEX_CURSOR.to_vec(),
        encode_head_marker(commit.target.height, identity),
    );
    Ok(entries)
}
