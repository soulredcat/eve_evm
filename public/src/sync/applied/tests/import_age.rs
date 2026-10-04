// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::import_fixtures::{import_chain, import_config};
use crate::{
    persistence::handoff::{
        observe_handoff, recovery_payload_bytes, reserve_recovery_payload, seal_recovery_payload,
        write_reserved_payload,
    },
    sync::applied::{
        admission::admit_prepared_publication::admit_prepared_publication,
        publication::build_applied_markers, recovery::prepare_charged_generation,
        state::applied_state_commit, *,
    },
};
use eve_storage::records::prospective_opaque_record_cursor;
use std::{sync::Arc, thread, time::Duration};

#[test]
fn imported_candidate_aged_after_preparation_rejects_final_admission_and_releases_its_leases() {
    let directory = tempfile::tempdir().unwrap();
    let chain = import_chain();
    let mut config = import_config(&directory.path().join("import"), &chain);
    config.public_budget.queue_age_ms = 1;
    let (mut owner, reader) = open_applied_state_service_with_mode(
        config,
        &chain.genesis,
        AppliedMode::AuthenticatedImport,
    )
    .ok()
    .unwrap();
    let original = capture_applied_state(&reader).unwrap();
    let charged = observe_estimated_working(&reader)
        .unwrap()
        .reserved_estimated_bytes;
    let cursor = owner.admitted_cursor;
    let mut buffer = reserve_recovery_payload(
        crate::sync::applied::admission::compact_pool(&owner).unwrap(),
        chain.records[0].len(),
    )
    .unwrap();
    write_reserved_payload(&mut buffer, &chain.records[0]).unwrap();
    let payload = seal_recovery_payload(buffer).unwrap();
    let generation = prepare_charged_generation(
        &original.generation,
        recovery_payload_bytes(&payload),
        &owner.config,
        &owner.reader.working,
    )
    .unwrap();
    let target = applied_state_commit(&generation.state).target.clone();
    let markers = build_applied_markers(&generation.state, 0).unwrap();
    let next = prospective_opaque_record_cursor(
        owner.effective_storage_identity,
        cursor,
        recovery_payload_bytes(&payload),
        owner.config.repository_budget.maximum_record_bytes,
    )
    .unwrap();
    let publication = Arc::new(AppliedPublication {
        generation,
        markers,
        durable_cursor: owner.durable_cursor,
        admitted_cursor: next,
        storage_failed: false,
        segmented_position: None,
    });
    thread::sleep(Duration::from_millis(5));
    assert!(matches!(
        admit_prepared_publication(&mut owner, publication, payload, next, target),
        Err(AppliedError::QueueLimit)
    ));
    assert!(Arc::ptr_eq(
        &original,
        &capture_applied_state(&reader).unwrap()
    ));
    assert_eq!(owner.admitted_cursor, cursor);
    assert!(owner.pending.is_empty());
    assert_eq!(
        observe_handoff(crate::sync::applied::admission::compact_pool(&owner).unwrap())
            .unwrap()
            .retained_bytes,
        0
    );
    assert_eq!(
        observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes,
        charged
    );
    drop(finish_applied_state_service(owner));
}
