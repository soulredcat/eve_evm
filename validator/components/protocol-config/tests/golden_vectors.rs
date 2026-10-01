// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "support/mod.rs"]
mod support;

use eve_protocol_config::{
    genesis::{encode_development_genesis, hash_development_genesis},
    headers::build_execution_header,
    native::{NATIVE_EVENTS_V1, NATIVE_FUNCTIONS_V1, native_event_topic, native_selector},
    network::LaunchMode,
    records::{SystemNamespace, hash_application_commitment, hash_system_key},
};

#[test]
fn protocol_v1_golden_bytes_hashes_and_abi_topics() {
    let genesis = support::genesis();
    let header = build_execution_header(&support::header()).unwrap();
    let vectors = serde_json::json!({
        "schema": "EVE_B0_PROTOCOL_V1_DEVELOPMENT_ONLY",
        "genesis_rlp": hex::encode(encode_development_genesis(LaunchMode::Development, &genesis).unwrap()),
        "genesis_hash": hex::encode(hash_development_genesis(LaunchMode::Development, &genesis).unwrap()),
        "header_rlp": hex::encode(alloy_rlp::encode(&header)),
        "header_hash": hex::encode(header.hash_slow()),
        "application_hash": hex::encode(hash_application_commitment(support::application()).unwrap().0),
        "fee_pool_system_key": hex::encode(hash_system_key(SystemNamespace::Fee, b"pool").unwrap()),
        "selectors": NATIVE_FUNCTIONS_V1.iter().map(|signature| (*signature,
            hex::encode(native_selector(signature)))).collect::<std::collections::BTreeMap<_, _>>(),
        "events": NATIVE_EVENTS_V1.iter().map(|signature| (*signature,
            hex::encode(native_event_topic(signature)))).collect::<std::collections::BTreeMap<_, _>>(),
    });
    let expected: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/protocol-v1.json")).unwrap();
    assert_eq!(vectors, expected);
}
