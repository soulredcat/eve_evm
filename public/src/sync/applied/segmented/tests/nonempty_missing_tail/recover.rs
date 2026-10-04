// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::fixtures::configuration;
use super::{
    lose_nonempty_marker::lose_nonempty_marker, nonempty_import_chain::nonempty_import_chain,
    read_peer_tail::read_peer_tail, store_peer_tail::store_peer_tail,
};
use crate::sync::applied::{resources::estimated_repository_read_charge, *};
use eve_development_fixtures::recovery::sender;
use eve_state::{
    decode_preflight_state_commit, development_state_budget, preflight_state_commit,
    required_state_commit_decode_reservation,
};
use eve_storage::records::{opaque_record_budget, opaque_record_cursor, read_opaque_record};
use std::sync::Arc;
#[test]
fn simulated_unsynced_nonempty_tail_is_read_from_actual_peer_wal_and_recovered_exactly_once() {
    let root = tempfile::tempdir().unwrap();
    let chain = nonempty_import_chain();
    let path = root.path().join("simulated-loss");
    let failed = lose_nonempty_marker(&root.path().join("original"), &path, &chain);
    let peer = store_peer_tail(&root.path().join("peer"), &chain, Some(&chain.records[1]));
    let (mut recovered, reader) =
        open_segmented_applied_state_service(configuration(&path, &chain), &chain.genesis)
            .ok()
            .unwrap();
    let restored = capture_applied_state(&reader).unwrap();
    assert_eq!(applied_commit(&restored), &chain.commits[1]);
    assert_eq!(
        applied_segmented_position(&restored).unwrap().missing_from,
        Some(2)
    );
    assert!(matches!(
        applied_readiness(&restored),
        eve_node_policy::PublicReadiness::NotReady(_)
    ));
    let (read_lease, bytes) = read_peer_tail(&peer, &reader);
    let bytes = bytes.unwrap();
    assert_eq!(bytes, chain.records[1]);
    assert_eq!(opaque_record_cursor(&peer).unwrap().sequence, 4);
    let commit_read = reserve_applied_working(
        &reader,
        estimated_repository_read_charge(&opaque_record_budget(&peer)).unwrap(),
    )
    .unwrap();
    let commit_row = read_opaque_record(&peer, 3).unwrap().unwrap();
    let preflight =
        preflight_state_commit(&commit_row.payload, &development_state_budget()).unwrap();
    let commit_decode = reserve_applied_working(
        &reader,
        required_state_commit_decode_reservation(&preflight).unwrap(),
    )
    .unwrap();
    let decoded = decode_preflight_state_commit(&preflight).unwrap();
    assert_eq!(decoded, chain.commits[2]);
    drop((decoded, commit_decode));
    try_apply_recovery_bytes(&mut recovered, &bytes).unwrap();
    let caught_up = capture_applied_state(&reader).unwrap();
    assert_eq!(applied_commit(&caught_up), &chain.commits[2]);
    assert_eq!(
        applied_commit(&caught_up).state.accounts[&sender()].nonce,
        2
    );
    assert_eq!(applied_commit(&caught_up).block.receipts.len(), 1);
    assert_eq!(
        applied_commit(&caught_up).state.system,
        chain.commits[2].state.system
    );
    assert!(try_apply_recovery_bytes(&mut recovered, &bytes).is_err());
    assert!(Arc::ptr_eq(
        &caught_up,
        &capture_applied_state(&reader).unwrap()
    ));
    assert_eq!(applied_commit(&restored), &chain.commits[1]);
    drop((bytes, commit_row, commit_read, read_lease));
    let finished = finish_applied_state_service(recovered);
    assert!(finished.acknowledgement_error.is_none());
    assert_eq!(retained_tail_len(&finished.unacknowledged_tail), 0);
    let final_view = finished.publication.as_ref().unwrap();
    assert_eq!(applied_commit(final_view), &chain.commits[2]);
    assert_eq!(applied_markers(final_view).durable_recovery.0, 2);
    assert_eq!(
        applied_segmented_position(final_view).unwrap().missing_from,
        None
    );
    assert_eq!(applied_cursors(final_view).1.sequence, 5);
    drop(finished.repository.unwrap());
    let (reopened, reread) =
        open_segmented_applied_state_service(configuration(&path, &chain), &chain.genesis)
            .ok()
            .unwrap();
    assert_eq!(
        applied_commit(&capture_applied_state(&reread).unwrap()),
        &chain.commits[2]
    );
    drop(finish_applied_state_service(reopened));
    // Original acknowledged data and its complete charged failed tail were never deleted.
    assert_eq!(retained_tail_len(&failed.unacknowledged_tail), 1);
    drop(failed);
}
