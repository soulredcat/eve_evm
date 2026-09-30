use super::{encode_account_record::encode_account_record, height_key};
use crate::state::types::{ACCOUNT_PREFIX, HASH_PREFIX, SLOT_PREFIX, SYSTEM_PREFIX};
use anyhow::{Result, anyhow};
use eve_state::{StateCommit, encode_system_record};
use std::collections::BTreeMap;

/// Expected complete current-domain rows; callers validate canonical state/bounds first.
pub(crate) fn build_current_state_entries(
    commit: &StateCommit,
) -> Result<BTreeMap<Vec<u8>, Vec<u8>>> {
    let mut rows = BTreeMap::new();
    for (address, account) in &commit.state.accounts {
        rows.insert(
            [ACCOUNT_PREFIX, address.as_slice()].concat(),
            encode_account_record(account),
        );
        for (slot, value) in &account.storage {
            rows.insert(
                [SLOT_PREFIX, address.as_slice(), &slot.to_be_bytes::<32>()].concat(),
                value.to_be_bytes::<32>().to_vec(),
            );
        }
    }
    for (key, record) in &commit.state.system {
        rows.insert(
            [SYSTEM_PREFIX, key.as_slice()].concat(),
            encode_system_record(record)
                .map_err(|error| anyhow!("invalid canonical system record: {error:?}"))?,
        );
    }
    for (height, hash) in &commit.state.block_hashes {
        rows.insert(height_key(HASH_PREFIX, *height), hash.0.to_vec());
    }
    Ok(rows)
}
