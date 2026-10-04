// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::fixtures::configuration;
use crate::{
    persistence::segmented::{install_segmented_record_pause, observe_segmented_parts},
    sync::applied::{
        resources::estimated_repository_read_charge, tests::import_fixtures::ImportChain,
        types::AppliedBackend, *,
    },
};
use eve_storage::records::{
    OpaqueRecordDisposition, compare_and_append_opaque_records, opaque_record_cursor,
    open_opaque_record_repository, read_opaque_record,
};
use std::{
    path::Path,
    thread,
    time::{Duration, Instant},
};

/// Copy actual H1 marker and H2 data ACK, excluding the unwritten marker; preserve original WAL and charged failed tail.
pub(super) fn lose_nonempty_marker(
    original: &Path,
    recovered: &Path,
    chain: &ImportChain,
) -> AppliedShutdown {
    assert!(chain.records.iter().all(|bytes| bytes.len() < 4_194_095));
    let (mut owner, reader) =
        open_segmented_applied_state_service(configuration(original, chain), &chain.genesis)
            .ok()
            .unwrap();
    let identity = owner.effective_storage_identity;
    let budget = owner.config.repository_budget;
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    let deadline = Instant::now() + Duration::from_secs(10);
    while poll_applied_durability(&mut owner)
        .unwrap()
        .durable_recovery
        .0
        < 1
    {
        assert!(
            Instant::now() < deadline,
            "actual H1 durable marker deadline"
        );
        thread::sleep(Duration::from_millis(5));
    }
    let first = capture_applied_state(&reader).unwrap();
    assert_eq!(applied_commit(&first), &chain.commits[1]);
    assert_eq!(applied_cursors(&first).1.sequence, 2);
    let AppliedBackend::Segmented { worker, pool, .. } = &owner.backend else {
        panic!("segmented")
    };
    let pool = std::sync::Arc::clone(pool);
    let (entered, resume) = install_segmented_record_pause(worker.as_ref().unwrap(), 1, true);
    try_apply_recovery_bytes(&mut owner, &chain.records[1]).unwrap();
    entered.recv_timeout(Duration::from_secs(10)).unwrap();
    let applied = capture_applied_state(&reader);
    resume.send(()).unwrap();
    let shutdown = finish_applied_state_service(owner);
    let applied = applied.unwrap();
    assert_eq!(applied_commit(&applied), &chain.commits[2]);
    assert_eq!(applied_markers(&applied).durable_recovery.0, 1);
    assert!(shutdown.repository.is_err());
    assert!(shutdown.acknowledgement_error.is_some());
    let progress = retained_tail_progress(&shutdown.unacknowledged_tail, 0).unwrap();
    assert_eq!(
        (
            progress.logical_target,
            progress.last_acknowledged_physical.sequence
        ),
        (2, 3)
    );
    assert!(progress.complete_marker.is_none());
    assert_eq!(observe_segmented_parts(&pool).unwrap().retained_parts, 2);
    let source = open_opaque_record_repository(original, identity, budget).unwrap();
    let mut destination = open_opaque_record_repository(recovered, identity, budget).unwrap();
    assert_eq!(
        opaque_record_cursor(&source).unwrap(),
        progress.last_acknowledged_physical
    );
    for sequence in 1..=3 {
        let _read =
            reserve_applied_working(&reader, estimated_repository_read_charge(&budget).unwrap())
                .unwrap();
        let record = read_opaque_record(&source, sequence).unwrap().unwrap();
        let expected = opaque_record_cursor(&destination).unwrap();
        let ack = compare_and_append_opaque_records(&mut destination, expected, &[record.payload])
            .unwrap();
        assert_eq!(ack.disposition, OpaqueRecordDisposition::NewlySynced);
        assert_eq!(ack.appended.sequence, sequence);
    }
    assert_eq!(
        opaque_record_cursor(&destination).unwrap(),
        progress.last_acknowledged_physical
    );
    assert!(read_opaque_record(&source, 4).unwrap().is_none());
    shutdown
}
