// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::fixtures::{configuration, logical_chain};
use crate::persistence::segmented::{observe_segmented_parts, observe_segmented_worker};
use crate::sync::applied::types::AppliedBackend;
use crate::sync::applied::*;
use std::sync::Arc;

#[test]
fn untrusted_expected_version_mismatch_refuses_before_marker_queue_or_ram_publication_and_releases_charges()
 {
    let directory = tempfile::tempdir().unwrap();
    let chain = logical_chain(false);
    let (mut owner, reader) = open_segmented_applied_state_service(
        configuration(&directory.path().join("segmented"), &chain),
        &chain.genesis,
    )
    .ok()
    .unwrap();
    let before = capture_applied_state(&reader).unwrap();
    let charge = observe_estimated_working(&reader)
        .unwrap()
        .reserved_estimated_bytes;
    let mut wrong_digest = chain.commits[1].target.clone();
    wrong_digest.content_digest.0[0] ^= 1;
    let mut wrong_height = chain.commits[1].target.clone();
    wrong_height.height += 1;
    for expected in [&wrong_digest, &wrong_height] {
        assert!(matches!(
            try_apply_recovery_bytes_matching_target(&mut owner, &chain.records[0], expected),
            Err(AppliedError::InvalidDurablePrefix)
        ));
        assert!(Arc::ptr_eq(
            &before,
            &capture_applied_state(&reader).unwrap()
        ));
        assert_eq!(owner.admitted_cursor, applied_cursors(&before).0);
        assert_eq!(owner.durable_cursor, applied_cursors(&before).1);
        assert!(owner.pending.is_empty());
        let AppliedBackend::Segmented { pool, worker, .. } = &owner.backend else {
            panic!("segmented");
        };
        assert_eq!(observe_segmented_parts(pool).unwrap().retained_parts, 0);
        assert_eq!(
            observe_segmented_worker(worker.as_ref().unwrap())
                .unwrap()
                .retained_logical_batches,
            0
        );
        assert_eq!(
            observe_estimated_working(&reader)
                .unwrap()
                .reserved_estimated_bytes,
            charge
        );
    }
    assert_eq!(
        try_apply_recovery_bytes_matching_target(
            &mut owner,
            &chain.records[0],
            &chain.commits[1].target
        )
        .unwrap()
        .applied
        .0,
        1
    );
    let shutdown = finish_applied_state_service(owner);
    assert!(shutdown.acknowledgement_error.is_none());
    assert_eq!(
        applied_markers(shutdown.publication.as_ref().unwrap())
            .durable_recovery
            .0,
        1
    );
}
