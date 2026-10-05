// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{StateVersion, encode_state_version};

pub const DOMAIN: &[u8] = b"EVE_IMPORT_V1";

pub fn replace(bytes: &[u8], field: usize, change: impl FnOnce(&[u8]) -> Vec<u8>) -> Vec<u8> {
    crate::codec_mutation::replace_field(bytes, DOMAIN.len(), field, change)
}

pub fn rlp_list(fields: &[Vec<u8>]) -> Vec<u8> {
    let mut output = Vec::new();
    alloy_rlp::Header {
        list: true,
        payload_length: fields.iter().map(Vec::len).sum(),
    }
    .encode(&mut output);
    for field in fields {
        output.extend_from_slice(field);
    }
    output
}

pub fn journal_with_operation(parent: &StateVersion, encoded_operation: Vec<u8>) -> Vec<u8> {
    rlp_list(&[
        alloy_rlp::encode(b"EVE_STATE_JOURNAL_V1".as_slice()),
        encode_state_version(parent).unwrap().to_vec(),
        alloy_rlp::encode(1_u64),
        rlp_list(&[encoded_operation]),
    ])
}
