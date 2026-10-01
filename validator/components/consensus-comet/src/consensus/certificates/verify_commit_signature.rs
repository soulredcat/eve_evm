// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CertificateError, ClassicalValidator, validator_address, verify_native_ed25519_signature,
};
use crate::{
    consensus::signing::{encode_vote_sign_bytes, normalize_timestamp},
    wire::tendermint::types::{Commit, CommitSig, Vote},
};

pub(super) fn verify_commit_signature(
    chain: &str,
    commit: &Commit,
    signature: &CommitSig,
    index: usize,
    validator: &ClassicalValidator,
) -> Result<bool, CertificateError> {
    if signature.block_id_flag == 1 {
        let time =
            normalize_timestamp(signature.timestamp.as_ref()).map_err(CertificateError::Signing)?;
        if !signature.validator_address.is_empty()
            || !signature.signature.is_empty()
            || time.seconds != -62_135_596_800
            || time.nanos != 0
        {
            return Err(CertificateError::InvalidSignatureLayout);
        }
        return Ok(false);
    }
    if !matches!(signature.block_id_flag, 2 | 3)
        || signature.validator_address.as_slice() != validator_address(&validator.public_key)
        || signature.signature.len() != 64
    {
        return Err(CertificateError::InvalidSignatureLayout);
    }
    let vote = Vote {
        r#type: 2,
        height: commit.height,
        round: commit.round,
        block_id: if signature.block_id_flag == 2 {
            commit.block_id.clone()
        } else {
            None
        },
        timestamp: signature.timestamp,
        validator_address: signature.validator_address.clone(),
        validator_index: i32::try_from(index)
            .map_err(|_| CertificateError::InvalidSignatureLayout)?,
        signature: signature.signature.clone(),
        extension: Vec::new(),
        extension_signature: Vec::new(),
    };
    let bytes = encode_vote_sign_bytes(chain, &vote).map_err(CertificateError::Signing)?;
    verify_native_ed25519_signature(&validator.public_key, &bytes, &signature.signature)?;
    Ok(signature.block_id_flag == 2)
}
