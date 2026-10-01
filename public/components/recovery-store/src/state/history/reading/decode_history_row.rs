// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::read_history_bytes::read_history_bytes;
use crate::state::history::HistorySnapshot;
use alloy_rlp::Decodable;
use anyhow::{Result, ensure};
pub(crate) fn decode_history_row<T: Decodable + alloy_rlp::Encodable>(
    snapshot: &HistorySnapshot<'_>,
    prefix: &[u8],
    height: u64,
    used: &mut usize,
) -> Result<T> {
    let bytes = read_history_bytes(snapshot, prefix, height, used)?;
    let mut remaining = bytes.as_slice();
    let result = T::decode(&mut remaining)?;
    ensure!(
        remaining.is_empty() && alloy_rlp::encode(&result) == bytes,
        "noncanonical history row"
    );
    Ok(result)
}
