// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod state_support;
use eve_storage::state::{
    commit_state, create_state_service, development_state_storage_budget, open_state_repository,
    read_state_service, state_reader,
};

#[test]
fn full_state_sync_survives_actual_child_exit_without_destructors() {
    let genesis = state_support::genesis();
    let first = state_support::next(&genesis, 1);
    let budget = development_state_storage_budget();
    if let Some(path) = std::env::var_os("EVE_FULL_STATE_CHILD_PATH") {
        let mut store =
            open_state_repository(std::path::Path::new(&path), &genesis, budget).unwrap();
        if std::env::var_os("EVE_FULL_STATE_BEFORE_WRITE").is_none() {
            commit_state(&mut store, &first).unwrap();
        }
        // Actual process exit bypasses Rust/DB destructors; hardware power loss is untested.
        std::process::exit(86);
    }
    for before_write in [true, false] {
        let directory = tempfile::tempdir().unwrap();
        drop(open_state_repository(directory.path(), &genesis, budget).unwrap());
        let mut child = std::process::Command::new(std::env::current_exe().unwrap());
        child
            .args([
                "--exact",
                "full_state_sync_survives_actual_child_exit_without_destructors",
            ])
            .env("EVE_FULL_STATE_CHILD_PATH", directory.path());
        if before_write {
            child.env("EVE_FULL_STATE_BEFORE_WRITE", "1");
        }
        assert_eq!(child.status().unwrap().code(), Some(86));
        let store = open_state_repository(directory.path(), &genesis, budget).unwrap();
        let service = create_state_service(state_reader(&store));
        assert_eq!(
            read_state_service(&service).unwrap().commit(),
            if before_write { &genesis } else { &first }
        );
    }
}
