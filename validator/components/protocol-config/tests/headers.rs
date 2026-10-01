// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "support/mod.rs"]
mod support;

use alloy_consensus::Header;
use alloy_rlp::Decodable;
use eve_protocol_config::headers::{HeaderError, build_execution_header};

#[test]
fn shanghai_header_roundtrips_and_keeps_fee_nonce_extra_data_and_newer_fork_fields_exact() {
    let input = support::header();
    let header = build_execution_header(&input).unwrap();
    let bytes = alloy_rlp::encode(&header);
    let mut slice = bytes.as_slice();
    assert_eq!(Header::decode(&mut slice).unwrap(), header);
    assert!(slice.is_empty());
    assert_eq!(header.extra_data.len(), 32);
    assert_eq!(&header.extra_data[..4], &[0, 0, 0, 1]);
    assert_eq!(&header.extra_data[4..], &[0x11; 28]);
    assert_eq!(header.base_fee_per_gas, Some(1_000_000_000));
    assert_eq!(
        header.withdrawals_root,
        Some(alloy_consensus::constants::EMPTY_WITHDRAWALS)
    );
    assert!(
        header.blob_gas_used.is_none()
            && header.excess_blob_gas.is_none()
            && header.parent_beacon_block_root.is_none()
            && header.requests_hash.is_none()
    );
    let mut same_second = input.clone();
    same_second.timestamp = same_second.parent_timestamp;
    assert!(build_execution_header(&same_second).is_ok());
}

#[test]
fn rejects_excess_gas_reversed_time_zero_fee_and_protocol() {
    let mut input = support::header();
    input.gas_used = input.gas_limit + 1;
    assert_eq!(
        build_execution_header(&input),
        Err(HeaderError::InvalidEnvironment)
    );
    input = support::header();
    input.timestamp = input.parent_timestamp - 1;
    assert_eq!(
        build_execution_header(&input),
        Err(HeaderError::InvalidEnvironment)
    );
    input = support::header();
    input.base_fee = 0;
    assert_eq!(
        build_execution_header(&input),
        Err(HeaderError::InvalidEnvironment)
    );
    input = support::header();
    input.protocol_version = 0;
    assert_eq!(
        build_execution_header(&input),
        Err(HeaderError::InvalidEnvironment)
    );
}
