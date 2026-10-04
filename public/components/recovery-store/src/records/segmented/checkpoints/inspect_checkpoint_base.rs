// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CHECKPOINT_BASE_MAX_VERSION_BYTES, CheckpointBaseError, CheckpointBaseLimits,
    CheckpointBaseView,
    inspection_types::CheckpointBaseInspection,
    read_checkpoint_base_array::read_checkpoint_base_array,
    read_checkpoint_base_metadata::read_checkpoint_base_metadata,
    types::{DOMAIN, HEADER_BYTES},
    validate_checkpoint_base_limits::validate_checkpoint_base_limits,
};
use eve_state::preflight_state_version;
use sha2::{Digest, Sha256};

/// Allocation-free local framing and canonical target-field admission only.
/// Hashes and returned metadata do not authenticate artifacts, state or finality.
pub fn inspect_checkpoint_base<'a>(
    bytes: &'a [u8],
    limits: &CheckpointBaseLimits,
) -> Result<CheckpointBaseInspection<'a>, CheckpointBaseError> {
    validate_checkpoint_base_limits(limits)?;
    if bytes.len() > limits.maximum_payload_bytes {
        return Err(CheckpointBaseError::LimitExceeded);
    }
    if bytes.len() < HEADER_BYTES + 33 || bytes.get(..22) != Some(DOMAIN.as_slice()) {
        return Err(CheckpointBaseError::MalformedEncoding);
    }
    if u16::from_be_bytes(read_checkpoint_base_array(bytes, 22)?) != 1 {
        return Err(CheckpointBaseError::UnsupportedVersion);
    }
    if bytes[24] != 1 {
        return Err(CheckpointBaseError::UnsupportedMode);
    }
    let length = usize::from(u16::from_be_bytes(read_checkpoint_base_array(bytes, 26)?));
    if length == 0 || length > CHECKPOINT_BASE_MAX_VERSION_BYTES {
        return Err(CheckpointBaseError::LimitExceeded);
    }
    let footer = HEADER_BYTES
        .checked_add(length)
        .ok_or(CheckpointBaseError::ArithmeticOverflow)?;
    if footer.checked_add(32) != Some(bytes.len()) {
        return Err(CheckpointBaseError::MalformedEncoding);
    }
    let supplied: [u8; 32] = read_checkpoint_base_array(bytes, footer)?;
    let actual: [u8; 32] = Sha256::digest(&bytes[..footer]).into();
    if actual != supplied {
        return Err(CheckpointBaseError::HashMismatch);
    }
    let version = &bytes[HEADER_BYTES..footer];
    preflight_state_version(version).map_err(|_| CheckpointBaseError::InvalidTarget)?;
    Ok(CheckpointBaseInspection {
        bytes,
        view: CheckpointBaseView {
            metadata: read_checkpoint_base_metadata(bytes)?,
            security_profile: bytes[25],
            target_version_bytes: version,
        },
    })
}
