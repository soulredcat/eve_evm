// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::fixtures::configuration;
use super::{
    lose_nonempty_marker::lose_nonempty_marker, nonempty_import_chain::nonempty_import_chain,
    read_peer_tail::read_peer_tail, store_peer_tail::store_peer_tail,
};
use crate::{
    persistence::segmented::observe_segmented_parts,
    sync::applied::{types::AppliedBackend, *},
};
use std::sync::Arc;
/// Selected peer corpus has either no H2 wire or only corrupted H3 authentication. Preserved originals are outside selection.
#[test]
fn unavailable_or_corrupt_only_peer_tail_preserves_missing_range_roots_fees_and_bounded_leases() {
    let root = tempfile::tempdir().unwrap();
    let chain = nonempty_import_chain();
    let mut invalid = chain.inputs[1].clone();
    invalid.lookahead.frame.commit.signatures[0].signature[0] ^= 1;
    let invalid = eve_finality_verifier::encode_logical_import_wire(
        &invalid,
        &eve_state::development_state_budget(),
    )
    .unwrap();
    for (name, tail) in [("missing", None), ("corrupt", Some(invalid.as_slice()))] {
        let case = root.path().join(name);
        std::fs::create_dir(&case).unwrap();
        let path = case.join("simulated-loss");
        let failed = lose_nonempty_marker(&case.join("original"), &path, &chain);
        let peer = store_peer_tail(&case.join("selected-peer"), &chain, tail);
        let (mut recovered, reader) =
            open_segmented_applied_state_service(configuration(&path, &chain), &chain.genesis)
                .ok()
                .unwrap();
        let before = capture_applied_state(&reader).unwrap();
        let baseline = observe_estimated_working(&reader)
            .unwrap()
            .reserved_estimated_bytes;
        let AppliedBackend::Segmented { pool, .. } = &recovered.backend else {
            panic!("segmented")
        };
        let pool = Arc::clone(pool);
        for _ in 0..32 {
            let (lease, bytes) = read_peer_tail(&peer, &reader);
            if let Some(bytes) = bytes {
                assert!(try_apply_recovery_bytes(&mut recovered, &bytes).is_err());
            }
            drop(lease);
            for bytes in [b"malformed-v2".as_slice(), chain.records[0].as_slice()] {
                assert!(try_apply_recovery_bytes(&mut recovered, bytes).is_err());
            }
            assert!(Arc::ptr_eq(
                &before,
                &capture_applied_state(&reader).unwrap()
            ));
            assert_eq!(recovered.admitted_cursor, applied_cursors(&before).0);
            assert!(recovered.pending.is_empty());
            assert_eq!(observe_segmented_parts(&pool).unwrap().retained_parts, 0);
            assert_eq!(
                observe_estimated_working(&reader)
                    .unwrap()
                    .reserved_estimated_bytes,
                baseline
            );
        }
        assert_eq!(applied_commit(&before), &chain.commits[1]);
        assert_eq!(applied_markers(&before).durable_recovery.0, 1);
        assert_eq!(
            applied_segmented_position(&before).unwrap().missing_from,
            Some(2)
        );
        assert!(matches!(
            applied_readiness(&before),
            eve_node_policy::PublicReadiness::NotReady(_)
        ));
        let shutdown = finish_applied_state_service(recovered);
        assert!(shutdown.acknowledgement_error.is_none());
        assert_eq!(
            applied_commit(shutdown.publication.as_ref().unwrap()),
            &chain.commits[1]
        );
        drop(shutdown.repository.unwrap());
        let (reopened, reread) =
            open_segmented_applied_state_service(configuration(&path, &chain), &chain.genesis)
                .ok()
                .unwrap();
        let reopened_view = capture_applied_state(&reread).unwrap();
        assert_eq!(applied_commit(&reopened_view), &chain.commits[1]);
        assert_eq!(
            applied_segmented_position(&reopened_view)
                .unwrap()
                .missing_from,
            Some(2)
        );
        drop(finish_applied_state_service(reopened));
        drop(failed);
    }
}
