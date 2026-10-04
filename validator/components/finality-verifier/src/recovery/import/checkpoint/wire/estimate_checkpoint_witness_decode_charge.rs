// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointWitnessWireError, CheckpointWitnessWireStats};
use eve_state::Bytes;

/// Conservative simultaneous decoded fields, maintained protobuf/state scratch and exact reencoding buffers.
pub(super) fn estimate_checkpoint_witness_decode_charge(
    stats: CheckpointWitnessWireStats,
) -> Result<usize, CheckpointWitnessWireError> {
    let mut elements = stats.native.transaction_count;
    let mut raw = stats.native.transaction_bytes;
    if let Some(execution) = stats.execution {
        elements = elements
            .checked_add(execution.transaction_count)
            .and_then(|count| count.checked_add(execution.receipt_count))
            .ok_or(CheckpointWitnessWireError::ArithmeticOverflow)?;
        raw = raw
            .checked_add(execution.transaction_bytes)
            .and_then(|bytes| bytes.checked_add(execution.receipt_bytes))
            .ok_or(CheckpointWitnessWireError::ArithmeticOverflow)?;
    }
    stats
        .encoded_bytes
        .checked_mul(16)
        .and_then(|bytes| bytes.checked_add(raw.checked_mul(4)?))
        .and_then(|bytes| {
            bytes.checked_add(elements.checked_mul(4 * std::mem::size_of::<Bytes>())?)
        })
        .and_then(|bytes| bytes.checked_add(stats.native.frame.signature_count.checked_mul(2_048)?))
        .and_then(|bytes| bytes.checked_add(stats.version_encoded_bytes.checked_mul(4)?))
        .and_then(|bytes| bytes.checked_add(stats.version_network_bytes.checked_mul(4)?))
        .and_then(|bytes| bytes.checked_add(262_144))
        .ok_or(CheckpointWitnessWireError::ArithmeticOverflow)
}
