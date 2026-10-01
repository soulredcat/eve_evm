// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

#[path = "../state_support/mod.rs"]
pub(crate) mod state_support;
use crate::state::{
    commit_state, create_state_service, development_state_storage_budget, open_state_repository,
    read_state_service, state_reader, types::SimulatedCommitFailure,
};

#[test]
fn simulated_write_or_lost_ack_fences_cache_and_writer_until_actual_reopen() {
    for after_sync in [false, true] {
        let directory = tempfile::tempdir().unwrap();
        let genesis = state_support::genesis();
        let first = state_support::next(&genesis, 1);
        let budget = development_state_storage_budget();
        let mut store = open_state_repository(directory.path(), &genesis, budget).unwrap();
        let service = create_state_service(state_reader(&store));
        assert_eq!(read_state_service(&service).unwrap().commit(), &genesis);
        store.simulated_failure = Some(if after_sync {
            SimulatedCommitFailure::AfterSuccessfulSync
        } else {
            SimulatedCommitFailure::BeforeWrite
        });
        let error = commit_state(&mut store, &first).unwrap_err();
        assert!(error.to_string().contains("SIMULATED"));
        assert!(commit_state(&mut store, &first).is_err());
        assert!(read_state_service(&service).is_err());
        drop(service);
        drop(store);
        let reopened = open_state_repository(directory.path(), &genesis, budget).unwrap();
        let service = create_state_service(state_reader(&reopened));
        assert_eq!(
            read_state_service(&service).unwrap().commit(),
            if after_sync { &first } else { &genesis }
        );
    }
}
