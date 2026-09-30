use crate::state::{encoding::height_key, types::HEADER_PREFIX};
use alloy_rlp::Decodable;
use anyhow::{Context, Result, ensure};
use eve_state::{ExecutionBlockHash, Header, StateCommit};
use rocksdb::{DB, SnapshotWithThreadMode};
use std::collections::{BTreeMap, BTreeSet};

/// Read only the required narrow headers from this same database sequence.
pub(crate) fn load_block_hash_history(
    snapshot: &SnapshotWithThreadMode<'_, DB>,
    commit: &StateCommit,
) -> Result<BTreeMap<u64, ExecutionBlockHash>> {
    let mut heights: BTreeSet<_> = commit.state.block_hashes.keys().copied().collect();
    heights.extend(commit.target.height.saturating_sub(256)..commit.target.height);
    let mut history = BTreeMap::new();
    for height in heights {
        if height == commit.target.height {
            continue;
        }
        ensure!(
            height < commit.target.height,
            "future execution hash in state history"
        );
        let bytes = snapshot
            .get(height_key(HEADER_PREFIX, height))?
            .context("required historical execution header is missing")?;
        let mut remaining = bytes.as_slice();
        let header =
            Header::decode(&mut remaining).context("malformed historical execution header")?;
        ensure!(
            remaining.is_empty() && header.number == height && alloy_rlp::encode(&header) == bytes,
            "noncanonical/misindexed historical execution header"
        );
        history.insert(height, ExecutionBlockHash(header.hash_slow()));
    }
    Ok(history)
}
