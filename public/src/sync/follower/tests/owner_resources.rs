// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{ValidatorFollowerSource, follow_next_validator_delta};
use crate::sync::applied::{
    capture_applied_state, finish_applied_state_service, observe_estimated_working,
    open_segmented_applied_state_service, reserve_applied_working,
    segmented::tests::fixtures::{configuration, logical_chain},
};
use eve_sync_client::{NativeRpcConfig, required_state_delta_download_reservation};
use std::{sync::Arc, time::Duration};

#[test]
fn follower_charges_its_actual_owner_even_when_a_foreign_reader_has_sufficient_capacity() {
    let directory = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let mut owner_config = configuration(&directory.path().join("owner"), &chain);
    owner_config
        .application
        .public_budget
        .maximum_working_state_bytes = 64 * 1_048_576;
    let (mut owner, owner_reader) =
        open_segmented_applied_state_service(owner_config, &chain.genesis)
            .ok()
            .unwrap();
    let (foreign, foreign_reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("foreign"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    let original = capture_applied_state(&owner_reader).unwrap();
    let before = observe_estimated_working(&owner_reader)
        .unwrap()
        .reserved_estimated_bytes;
    let rpc = NativeRpcConfig {
        address: "127.0.0.1:1".parse().unwrap(),
        timeout: Duration::from_secs(1),
        maximum_request_bytes: 16_384,
        maximum_response_bytes: 262_144,
    };
    let requested = required_state_delta_download_reservation(rpc).unwrap();
    let foreign_lease = reserve_applied_working(&foreign_reader, requested).unwrap();
    // The API derives its reader from owner; another service's budget cannot be supplied.
    let result = follow_next_validator_delta(
        &mut owner,
        ValidatorFollowerSource {
            rpc,
            maximum_chunk_bytes: 32_768,
        },
    );
    assert!(result.unwrap_err().to_string().contains("RESOURCE_LIMIT"));
    assert!(Arc::ptr_eq(
        &original,
        &capture_applied_state(&owner_reader).unwrap()
    ));
    assert_eq!(
        observe_estimated_working(&owner_reader)
            .unwrap()
            .reserved_estimated_bytes,
        before
    );
    drop(foreign_lease);
    drop(finish_applied_state_service(owner));
    drop(finish_applied_state_service(foreign));
}
