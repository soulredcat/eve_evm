// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#![cfg(test)]

use eve_state::BlockPayload;

/// Exact pre-extraction block-field construction, retained only as a compatibility oracle.
pub fn encode_legacy_block_payload(block: &BlockPayload) -> Vec<u8> {
    let list = |fields: &[Vec<u8>]| {
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
    };
    let transactions = block
        .transactions
        .iter()
        .map(|bytes| alloy_rlp::encode(bytes.as_ref()))
        .collect::<Vec<_>>();
    let receipts = block
        .receipts
        .iter()
        .map(|bytes| alloy_rlp::encode(bytes.as_ref()))
        .collect::<Vec<_>>();
    list(&[
        alloy_rlp::encode(&block.header),
        list(&transactions),
        list(&receipts),
    ])
}
