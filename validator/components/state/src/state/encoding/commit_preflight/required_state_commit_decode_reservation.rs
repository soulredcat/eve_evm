// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::StateCommitPreflight;
use crate::{Bytes, Header, StateError};

/// Actual-count logical map/blob/root/encoding envelope; caller reserves real capacity before decode.
/// Includes complete canonical reencoding/output intermediates, not input assembly or allocator/RSS.
pub fn required_state_commit_decode_reservation(
    preflight: &StateCommitPreflight<'_>,
) -> Result<usize, StateError> {
    let stats = preflight.stats;
    let networks = stats
        .parent_network_bytes
        .checked_add(stats.target_network_bytes)
        .and_then(|bytes| bytes.checked_add(stats.state_network_bytes))
        .ok_or(StateError::ArithmeticOverflow)?;
    let raw = stats
        .transaction_bytes
        .checked_add(stats.receipt_bytes)
        .ok_or(StateError::ArithmeticOverflow)?;
    let terms = [
        (stats.accounts, 512),
        (stats.storage_slots, 256),
        (stats.codes, 256),
        (stats.code_bytes, 2),
        (stats.system_records, 512),
        (stats.system_payload_bytes, 2),
        (stats.system_leaf_count, 2 * std::mem::size_of::<Bytes>()),
        (stats.history_entries, 128),
        (networks, 2),
        (stats.header_payload_bytes, 2),
        (stats.header_leaf_count, 2 * std::mem::size_of::<Bytes>()),
        (raw, 2),
        (stats.block_vector_allocation_bytes, 1),
        (stats.encoded_bytes, 6),
        (stats.bounded_codec_scratch_bytes, 1),
    ];
    let base = std::mem::size_of::<Header>()
        .checked_mul(2)
        .and_then(|bytes| bytes.checked_add(2_097_152))
        .ok_or(StateError::ArithmeticOverflow)?;
    terms
        .into_iter()
        .try_fold(base, |sum, (count, factor)| {
            sum.checked_add(count.checked_mul(factor)?)
        })
        .ok_or(StateError::ArithmeticOverflow)
}
