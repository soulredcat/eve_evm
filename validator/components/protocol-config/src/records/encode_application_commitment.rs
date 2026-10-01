// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::encoding::encode_list;

use super::{ApplicationCommitmentInput, RecordError};

pub fn encode_application_commitment(
    input: ApplicationCommitmentInput,
) -> Result<Vec<u8>, RecordError> {
    if input.protocol_version == 0 || input.execution_height == 0 {
        return Err(RecordError::InvalidVersion);
    }
    Ok(encode_list(&[
        alloy_rlp::encode(b"EVE_APP_V1".as_slice()),
        alloy_rlp::encode(input.genesis.0),
        alloy_rlp::encode(input.protocol_version),
        alloy_rlp::encode(input.execution_height),
        alloy_rlp::encode(input.evm_root.0),
        alloy_rlp::encode(input.system_root.0),
        alloy_rlp::encode(input.execution_hash.0),
    ]))
}
