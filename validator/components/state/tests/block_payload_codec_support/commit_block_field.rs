// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#![cfg(test)]

/// Locate the fifth whole-commit field in a known-valid fixture without reencoding state data.
pub fn commit_block_field(bytes: &[u8]) -> &[u8] {
    let mut remaining = bytes;
    let outer = alloy_rlp::Header::decode(&mut remaining).unwrap();
    assert!(outer.list);
    let (mut fields, trailing) = remaining.split_at(outer.payload_length);
    assert!(trailing.is_empty());
    for _ in 0..4 {
        let before = fields;
        let header = alloy_rlp::Header::decode(&mut fields).unwrap();
        let prefix_length = before.len() - fields.len();
        fields = &before[prefix_length + header.payload_length..];
    }
    fields
}
