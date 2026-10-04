// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::application::{
    serving::tests::fixtures::advance, tests::fixture::test_application,
};
use eve_storage::state::{
    capture_state_snapshot, read_snapshot_commit, read_state_service, state_reader,
};
#[test]
fn held_cache_and_database_snapshot_preserve_one_exact_target_after_new_commit() {
    let mut fixture = test_application(false);
    let first = advance(&mut fixture);
    let cached = read_state_service(&fixture.application.service).unwrap();
    let reader = state_reader(&fixture.application.repository);
    let snapshot = capture_state_snapshot(&reader).unwrap();
    assert_eq!(snapshot.version(), &cached.commit().target);
    let second = advance(&mut fixture);
    assert_eq!(cached.commit(), &first);
    assert_eq!(snapshot.version(), &first.target);
    assert_eq!(read_snapshot_commit(&snapshot, 1).unwrap().unwrap(), first);
    assert!(read_snapshot_commit(&snapshot, 2).unwrap().is_none());
    assert_eq!(
        read_state_service(&fixture.application.service)
            .unwrap()
            .commit(),
        &second
    );
}
