// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

pub(crate) fn encode_list(fields: &[Vec<u8>]) -> Vec<u8> {
    let mut bytes = Vec::new();
    alloy_rlp::Header {
        list: true,
        payload_length: fields.iter().map(Vec::len).sum(),
    }
    .encode(&mut bytes);
    for field in fields {
        bytes.extend_from_slice(field);
    }
    bytes
}
