// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CertificateError,
    hashing::{
        hash_byte_slices::hash_byte_slices,
        types::{BytesValue, Int64Value, StringValue},
    },
    validate_consensus_header::validate_consensus_header,
};
use crate::{consensus::signing::normalize_timestamp, wire::tendermint::types::Header};
use prost::Message;

/// Native 14-field Header.Hash; distinct from the EVM header's Keccak/RLP hash.
pub fn hash_consensus_header(header: &Header) -> Result<[u8; 32], CertificateError> {
    validate_consensus_header(header)?;
    let mut prior = header.last_block_id.clone().unwrap_or_default();
    // Native gogo BlockID's nonnullable PartSetHeader is encoded even when empty.
    prior.part_set_header.get_or_insert_default();
    let mut leaves: Vec<_> = [
        header
            .version
            .as_ref()
            .ok_or(CertificateError::InvalidHeaderVersion)?
            .encode_to_vec(),
        StringValue {
            value: header.chain_id.clone(),
        }
        .encode_to_vec(),
        Int64Value {
            value: header.height,
        }
        .encode_to_vec(),
        normalize_timestamp(header.time.as_ref())
            .map_err(CertificateError::Signing)?
            .encode_to_vec(),
        prior.encode_to_vec(),
    ]
    .into_iter()
    .collect();
    leaves.extend(
        [
            &header.last_commit_hash,
            &header.data_hash,
            &header.validators_hash,
            &header.next_validators_hash,
            &header.consensus_hash,
            &header.app_hash,
            &header.last_results_hash,
            &header.evidence_hash,
            &header.proposer_address,
        ]
        .map(|value| {
            BytesValue {
                value: value.clone(),
            }
            .encode_to_vec()
        }),
    );
    Ok(hash_byte_slices(&leaves))
}
