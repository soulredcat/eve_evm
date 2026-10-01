// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use alloy_rlp::Header;

pub(crate) fn encode_list(fields: &[Vec<u8>]) -> Vec<u8> {
    let payload_length = fields.iter().map(Vec::len).sum();
    let mut output = Vec::new();
    Header {
        list: true,
        payload_length,
    }
    .encode(&mut output);
    for field in fields {
        output.extend_from_slice(field);
    }
    output
}
