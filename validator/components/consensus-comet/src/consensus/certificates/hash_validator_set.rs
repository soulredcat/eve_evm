// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CertificateError, ClassicalValidator, hashing::hash_byte_slices::hash_byte_slices,
    validate_validator_set::validate_validator_set,
};
use crate::wire::tendermint::{
    crypto::{PublicKey, public_key::Sum},
    types::SimpleValidator,
};
use prost::Message;

/// Hash a native ordered set using exact SimpleValidator protobuf leaves.
pub fn hash_validator_set(validators: &[ClassicalValidator]) -> Result<[u8; 32], CertificateError> {
    validate_validator_set(validators, true)?;
    let leaves: Vec<_> = validators
        .iter()
        .map(|validator| {
            SimpleValidator {
                pub_key: Some(PublicKey {
                    sum: Some(Sum::Ed25519(validator.public_key.to_vec())),
                }),
                voting_power: validator.voting_power,
            }
            .encode_to_vec()
        })
        .collect();
    Ok(hash_byte_slices(&leaves))
}
