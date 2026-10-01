// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use std::collections::BTreeMap;

use alloy_primitives::B256;
use alloy_trie::{HashBuilder, Nibbles};
use eve_protocol_config::records::{
    SystemRecord, SystemStateRoot, encode_system_record, hash_system_key,
};

use crate::StateError;

pub fn compute_system_root(
    records: &BTreeMap<B256, SystemRecord>,
) -> Result<SystemStateRoot, StateError> {
    let mut builder = HashBuilder::default();
    for (key, record) in records {
        if hash_system_key(record.namespace, &record.logical_key)
            .map_err(StateError::SystemRecord)?
            != *key
        {
            return Err(StateError::SystemKeyMismatch);
        }
        let encoded = encode_system_record(record).map_err(StateError::SystemRecord)?;
        builder.add_leaf(Nibbles::unpack(*key), &encoded);
    }
    Ok(SystemStateRoot(builder.root()))
}
