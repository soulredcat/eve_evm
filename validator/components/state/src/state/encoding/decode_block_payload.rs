// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_block_payload_from::decode_block_payload_from, encode_block_payload};
use crate::{BlockPayload, StateBudget, StateError};

/// Decode one bounded canonical wire payload; execution and authentication remain separate.
pub fn decode_block_payload(
    bytes: &[u8],
    budget: &StateBudget,
) -> Result<BlockPayload, StateError> {
    if bytes.len() > budget.maximum_commit_bytes {
        return Err(StateError::BudgetExceeded);
    }
    let mut remaining = bytes;
    let block = decode_block_payload_from(&mut remaining, budget)?;
    if !remaining.is_empty() {
        return Err(StateError::MalformedEncoding);
    }
    if encode_block_payload(&block, budget)? != bytes {
        return Err(StateError::NonCanonicalEncoding);
    }
    Ok(block)
}
