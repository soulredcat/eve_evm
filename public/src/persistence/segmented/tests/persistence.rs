// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::*;
use super::{
    fixtures::{batch, fixture},
    pause,
};
use eve_storage::records::segmented::{
    preflight_segmented_record, segmented_marker_reference_at, segmented_marker_view,
};
use eve_storage::records::{
    opaque_record_cursor, open_opaque_record_repository, read_opaque_record,
};
use std::time::Duration;
#[test]
fn six_synced_segments_precede_marker_and_only_validated_logical_ack_releases_full_tail() {
    let fixture = fixture();
    let bytes = vec![0x31; 21_025_569];
    let sealed = batch(&fixture.pool, fixture.parent, fixture.parent.cursor, &bytes);
    let worker = start_segmented_worker(fixture.repository, fixture.pool.clone(), fixture.parent)
        .unwrap_or_else(|_| panic!("valid worker"));
    let (entered, resume) = pause(&worker, 6, false);
    let ticket = try_submit_segmented_batch(&worker, fixture.parent.cursor, sealed)
        .ok()
        .unwrap();
    entered.recv_timeout(Duration::from_secs(20)).unwrap();
    let before = try_receive_segmented_ack(&ticket);
    let charged = observe_segmented_parts(&fixture.pool).unwrap();
    let scratch = observe_segmented_worker(&worker).unwrap();
    resume.send(()).unwrap();
    assert_eq!(before, Ok(None));
    assert_eq!(charged.retained_parts, 4);
    assert!(scratch.active_estimated_scratch_bytes > 8 * 1_048_576);
    let shutdown = finish_segmented_worker(worker);
    let ack = try_receive_segmented_ack(&ticket).unwrap().unwrap();
    assert_eq!(ack.identity.target_height, 1);
    assert_eq!(ack.marker_cursor.sequence, 7);
    let repository = shutdown.repository.unwrap();
    assert_eq!(
        opaque_record_cursor(&repository).unwrap(),
        ack.marker_cursor
    );
    let marker = read_opaque_record(&repository, 7).unwrap().unwrap();
    let preflight = preflight_segmented_record(&marker.payload, &fixture.pool.codec).unwrap();
    let view = segmented_marker_view(&preflight).unwrap();
    for index in 0..6 {
        assert_eq!(
            segmented_marker_reference_at(&view, index).unwrap(),
            ack.references[index]
        );
    }
    drop(repository);
    let reopened =
        open_opaque_record_repository(&fixture.path, fixture.namespace, fixture.budget).unwrap();
    assert_eq!(opaque_record_cursor(&reopened).unwrap(), ack.marker_cursor);
    drop(shutdown.tails);
    drop(ticket);
    assert_eq!(
        observe_segmented_parts(&fixture.pool)
            .unwrap()
            .retained_parts,
        0
    );
}
