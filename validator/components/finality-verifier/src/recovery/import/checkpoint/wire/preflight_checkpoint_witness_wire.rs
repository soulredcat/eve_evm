// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::CheckpointLimits;
use super::{
    CheckpointWitnessWireError, CheckpointWitnessWireKind, CheckpointWitnessWirePreflight,
    CheckpointWitnessWireStats, types::DOMAIN,
    validate_checkpoint_witness_wire_limits::validate_checkpoint_witness_wire_limits,
};
use crate::recovery::{
    bounds::types::{
        MAXIMUM_EXECUTION_DATA_BYTES, MAXIMUM_NATIVE_DATA_BYTES, MAXIMUM_NATIVE_FRAME_BYTES,
        MAXIMUM_TRANSACTIONS, MAXIMUM_VERSION_BYTES,
    },
    decoding::take_length_prefixed::take_length_prefixed,
    import::wire::{scan_import_execution_stats_with_limit, scan_import_lookahead_stats},
};
use eve_state::{StateBudget, preflight_state_version};

/// Allocation-free framing/count/version-width scan through the existing native and state-owned scanners.
pub fn preflight_checkpoint_witness_wire<'a>(
    bytes: &'a [u8],
    budget: &StateBudget,
    limits: CheckpointLimits,
) -> Result<CheckpointWitnessWirePreflight<'a>, CheckpointWitnessWireError> {
    validate_checkpoint_witness_wire_limits(budget, limits)?;
    if bytes.len() > limits.maximum_witness_bytes {
        return Err(CheckpointWitnessWireError::BudgetExceeded);
    }
    if !bytes.starts_with(DOMAIN) {
        return Err(CheckpointWitnessWireError::MalformedEncoding);
    }
    let mut remaining = &bytes[DOMAIN.len()..];
    let (&tag, tail) = remaining
        .split_first()
        .ok_or(CheckpointWitnessWireError::MalformedEncoding)?;
    remaining = tail;
    let kind = match tag {
        1 => CheckpointWitnessWireKind::Execution,
        2 => CheckpointWitnessWireKind::Lookahead,
        _ => return Err(CheckpointWitnessWireError::MalformedEncoding),
    };
    let native_limit = MAXIMUM_NATIVE_DATA_BYTES
        .checked_add(MAXIMUM_NATIVE_FRAME_BYTES)
        .and_then(|length| length.checked_add(8 + 4 * MAXIMUM_TRANSACTIONS))
        .ok_or(CheckpointWitnessWireError::ArithmeticOverflow)?;
    let native = take_length_prefixed(&mut remaining, native_limit)
        .map_err(CheckpointWitnessWireError::Recovery)?;
    let native_stats =
        scan_import_lookahead_stats(native).map_err(CheckpointWitnessWireError::ImportWire)?;
    if native_stats.transaction_count > budget.maximum_journal_operations {
        return Err(CheckpointWitnessWireError::BudgetExceeded);
    }
    let (version, version_network_bytes, execution, execution_stats) = match kind {
        CheckpointWitnessWireKind::Execution => {
            let version = take_length_prefixed(&mut remaining, MAXIMUM_VERSION_BYTES)
                .map_err(CheckpointWitnessWireError::Recovery)?;
            let network_bytes =
                preflight_state_version(version).map_err(CheckpointWitnessWireError::State)?;
            let execution = take_length_prefixed(
                &mut remaining,
                MAXIMUM_EXECUTION_DATA_BYTES.min(budget.maximum_commit_bytes),
            )
            .map_err(CheckpointWitnessWireError::Recovery)?;
            let stats = scan_import_execution_stats_with_limit(
                execution,
                budget,
                MAXIMUM_EXECUTION_DATA_BYTES,
            )
            .map_err(CheckpointWitnessWireError::ImportWire)?;
            (Some(version), network_bytes, Some(execution), Some(stats))
        }
        CheckpointWitnessWireKind::Lookahead => (None, 0, None, None),
    };
    if !remaining.is_empty() {
        return Err(CheckpointWitnessWireError::MalformedEncoding);
    }
    Ok(CheckpointWitnessWirePreflight {
        bytes,
        budget: *budget,
        limits,
        kind,
        native,
        version,
        execution,
        stats: CheckpointWitnessWireStats {
            encoded_bytes: bytes.len(),
            version_encoded_bytes: version.map_or(0, <[u8]>::len),
            version_network_bytes,
            native: native_stats,
            execution: execution_stats,
        },
    })
}
