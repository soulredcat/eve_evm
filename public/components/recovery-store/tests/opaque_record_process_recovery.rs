// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod opaque_record_support;
use eve_storage::records::{
    compare_and_append_opaque_records, opaque_record_cursor, open_opaque_record_repository,
    read_opaque_record,
};
use opaque_record_support::{budget, identity};

#[test]
fn opaque_synced_append_survives_actual_child_exit_without_destructors() {
    if let Some(path) = std::env::var_os("EVE_OPAQUE_EXIT_CHILD_PATH") {
        let mut store =
            open_opaque_record_repository(std::path::Path::new(&path), identity(), budget())
                .unwrap();
        let expected = opaque_record_cursor(&store).unwrap();
        if std::env::var_os("EVE_OPAQUE_EXIT_BEFORE_APPEND").is_none() {
            compare_and_append_opaque_records(&mut store, expected, &[vec![1], vec![2]]).unwrap();
        }
        // Real process exit skips Rust/DB destructors. It does not emulate lost drive power.
        std::process::exit(91);
    }
    for before_append in [true, false] {
        let temporary = tempfile::tempdir().unwrap();
        let path = temporary.path().join("opaque");
        drop(open_opaque_record_repository(&path, identity(), budget()).unwrap());
        let mut child = std::process::Command::new(std::env::current_exe().unwrap());
        child
            .args([
                "--exact",
                "opaque_synced_append_survives_actual_child_exit_without_destructors",
            ])
            .env("EVE_OPAQUE_EXIT_CHILD_PATH", &path);
        if before_append {
            child.env("EVE_OPAQUE_EXIT_BEFORE_APPEND", "1");
        }
        assert_eq!(child.status().unwrap().code(), Some(91));
        let reopened = open_opaque_record_repository(&path, identity(), budget()).unwrap();
        assert_eq!(
            opaque_record_cursor(&reopened).unwrap().sequence,
            if before_append { 0 } else { 2 }
        );
        if !before_append {
            assert_eq!(
                read_opaque_record(&reopened, 1).unwrap().unwrap().payload,
                vec![1]
            );
            assert_eq!(
                read_opaque_record(&reopened, 2).unwrap().unwrap().payload,
                vec![2]
            );
        }
    }
}

#[test]
fn opaque_database_owner_is_exclusive_across_actual_processes() {
    if let Some(path) = std::env::var_os("EVE_OPAQUE_LOCK_CHILD_PATH") {
        assert!(
            open_opaque_record_repository(std::path::Path::new(&path), identity(), budget())
                .is_err()
        );
        std::process::exit(92);
    }
    let temporary = tempfile::tempdir().unwrap();
    let path = temporary.path().join("opaque");
    let store = open_opaque_record_repository(&path, identity(), budget()).unwrap();
    assert!(open_opaque_record_repository(&path, identity(), budget()).is_err());
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .args([
            "--exact",
            "opaque_database_owner_is_exclusive_across_actual_processes",
        ])
        .env("EVE_OPAQUE_LOCK_CHILD_PATH", &path)
        .status()
        .unwrap();
    assert_eq!(status.code(), Some(92));
    drop(store);
    assert!(open_opaque_record_repository(&path, identity(), budget()).is_ok());
}
