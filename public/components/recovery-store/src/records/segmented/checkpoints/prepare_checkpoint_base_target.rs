// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CHECKPOINT_BASE_MAX_VERSION_BYTES, CheckpointBaseError, CheckpointBaseLimits,
    PreparedCheckpointBaseTarget, required_checkpoint_base_encoding_reservation,
};
use eve_state::{StateVersion, encode_state_version};

/// Bound inputs before invoking the canonical codec. Caller holds the actual lease
/// throughout preparation, encoding and every retained prepared target/payload.
pub fn prepare_checkpoint_base_target(
    version: &StateVersion,
    limits: &CheckpointBaseLimits,
    reserved_bytes: usize,
) -> Result<PreparedCheckpointBaseTarget, CheckpointBaseError> {
    if reserved_bytes < required_checkpoint_base_encoding_reservation(limits)? {
        return Err(CheckpointBaseError::InsufficientReservation);
    }
    // Local defensive ceiling only; eve-state retains its stricter canonical identity policy.
    if version.identity.network_name.len() > 1_024 {
        return Err(CheckpointBaseError::LimitExceeded);
    }
    let encoded = encode_state_version(version).map_err(|_| CheckpointBaseError::InvalidTarget)?;
    if encoded.is_empty() || encoded.len() > CHECKPOINT_BASE_MAX_VERSION_BYTES {
        return Err(CheckpointBaseError::LimitExceeded);
    }
    Ok(PreparedCheckpointBaseTarget {
        encoded,
        height: version.height,
        security_profile: version.identity.security_profile as u8,
    })
}
