// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    types::{
        MAXIMUM_BLOCK_ID_BYTES, MAXIMUM_NATIVE_COMMIT_BYTES, MAXIMUM_NATIVE_FRAME_BYTES,
        MAXIMUM_NATIVE_HEADER_BYTES,
    },
    validate_native_block_id_bounds::validate_native_block_id_bounds,
    validate_native_header_bounds::validate_native_header_bounds,
};
use crate::recovery::{NativeFrame, RecoveryError};
use eve_consensus_comet::consensus::certificates::MAX_DEVELOPMENT_VALIDATORS;
use prost::Message;

pub(crate) fn measure_native_frame_bytes(frame: &NativeFrame) -> Result<usize, RecoveryError> {
    validate_native_block_id_bounds(&frame.block_id)?;
    validate_native_header_bounds(&frame.header)?;
    if let Some(id) = &frame.commit.block_id {
        validate_native_block_id_bounds(id)?;
    }
    if frame.commit.signatures.len() > MAX_DEVELOPMENT_VALIDATORS
        || frame.commit.signatures.iter().any(|signature| {
            signature.validator_address.len() > 20 || signature.signature.len() > 64
        })
    {
        return Err(RecoveryError::BudgetExceeded);
    }
    let id = frame.block_id.encoded_len();
    let header = frame.header.encoded_len();
    let commit = frame.commit.encoded_len();
    if id > MAXIMUM_BLOCK_ID_BYTES
        || header > MAXIMUM_NATIVE_HEADER_BYTES
        || commit > MAXIMUM_NATIVE_COMMIT_BYTES
    {
        return Err(RecoveryError::BudgetExceeded);
    }
    let size = 12 + id + header + commit;
    if size > MAXIMUM_NATIVE_FRAME_BYTES {
        return Err(RecoveryError::BudgetExceeded);
    }
    Ok(size)
}
