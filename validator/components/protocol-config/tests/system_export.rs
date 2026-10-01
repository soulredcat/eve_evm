// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "support/mod.rs"]
mod support;

use alloy_primitives::Bytes;
use eve_protocol_config::records::{RecordError, encode_system_export};

#[test]
fn export_orders_canonical_keys_and_rejects_duplicates_and_unbounded_chunks() {
    let first = support::fee_record();
    let mut second = first.clone();
    second.logical_key = Bytes::from_static(b"other");
    assert_eq!(
        encode_system_export(&[first.clone(), second.clone()]).unwrap(),
        encode_system_export(&[second, first.clone()]).unwrap()
    );
    assert_eq!(
        encode_system_export(&[first.clone(), first.clone()]),
        Err(RecordError::DuplicateKey)
    );
    assert_eq!(
        encode_system_export(&vec![first; 1_025]),
        Err(RecordError::InvalidRecord)
    );
}
