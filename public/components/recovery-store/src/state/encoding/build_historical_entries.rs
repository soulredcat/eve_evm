// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{encode_root_record::encode_root_record, height_key};
use crate::state::types::{
    COMMIT_PREFIX, HEADER_PREFIX, ID_PREFIX, RECEIPT_PREFIX, ROOT_PREFIX, TX_PREFIX,
};
use alloy_primitives::B256;
use eve_state::StateCommit;
use std::collections::BTreeMap;

pub(crate) fn build_historical_entries(
    commit: &StateCommit,
    bytes: Vec<u8>,
    identity: B256,
) -> BTreeMap<Vec<u8>, Vec<u8>> {
    let height = commit.target.height;
    BTreeMap::from([
        (height_key(COMMIT_PREFIX, height), bytes),
        (height_key(ID_PREFIX, height), identity.to_vec()),
        (
            height_key(HEADER_PREFIX, height),
            alloy_rlp::encode(&commit.block.header),
        ),
        (
            height_key(TX_PREFIX, height),
            alloy_rlp::encode(&commit.block.transactions),
        ),
        (
            height_key(RECEIPT_PREFIX, height),
            alloy_rlp::encode(&commit.block.receipts),
        ),
        (
            height_key(ROOT_PREFIX, height),
            encode_root_record(&commit.target),
        ),
    ])
}
