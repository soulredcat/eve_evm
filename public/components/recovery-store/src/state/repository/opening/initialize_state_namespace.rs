// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::{
    StateStorageBudget,
    encoding::{build_current_state_entries, build_historical_entries, encode_head_marker},
    types::{ACTIVE_KEY, CODE_PREFIX, GENESIS_KEY, HEAD_KEY, SCHEMA_BYTES, SCHEMA_KEY},
};
use anyhow::{Result, anyhow, ensure};
use eve_state::{StateCommit, compute_commit_identity, encode_state_commit};
use rocksdb::{DB, WriteBatch, WriteOptions};

pub(crate) fn initialize_state_namespace(
    database: &DB,
    genesis: &StateCommit,
    budget: &StateStorageBudget,
    active: bool,
) -> Result<()> {
    ensure!(
        database.latest_sequence_number() == 0,
        "nonempty database has no full-state schema marker"
    );
    ensure!(
        genesis.parent.is_none() && genesis.target.height == 0,
        "full-state bootstrap requires exact genesis commit"
    );
    let bytes = encode_state_commit(genesis, &budget.logical)
        .map_err(|error| anyhow!("invalid genesis commit: {error:?}"))?;
    let identity = compute_commit_identity(genesis, &budget.logical)
        .map_err(|error| anyhow!("invalid genesis identity: {error:?}"))?;
    let mut batch = WriteBatch::default();
    batch.put(SCHEMA_KEY, SCHEMA_BYTES);
    batch.put(ACTIVE_KEY, [u8::from(active)]);
    batch.put(GENESIS_KEY, &bytes);
    for (key, value) in build_current_state_entries(genesis)? {
        batch.put(key, value);
    }
    for (hash, code) in &genesis.state.codes {
        batch.put([CODE_PREFIX, hash.as_slice()].concat(), code.as_ref());
    }
    for (key, value) in build_historical_entries(genesis, bytes, identity) {
        batch.put(key, value);
    }
    batch.put(HEAD_KEY, encode_head_marker(0, identity));
    ensure!(
        batch.size_in_bytes() <= budget.maximum_commit_bytes,
        "genesis full-state batch exceeds byte budget"
    );
    let mut writes = WriteOptions::default();
    writes.set_sync(true);
    writes.disable_wal(false);
    database.write_opt(batch, &writes)?;
    Ok(())
}
