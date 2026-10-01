// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod common;

use anyhow::Result;
use eve_storage::recovery::opening::open_record_store;
use eve_storage::recovery::records::{commit_record_batch, read_record, read_record_cursor};

#[test]
fn synced_batch_survives_process_exit_without_database_destructors() -> Result<()> {
    if let Some(path) = std::env::var_os("EVE_STORAGE_TEST_CHILD_PATH") {
        let mut store = open_record_store(
            std::path::Path::new(&path),
            common::identity(),
            common::budget(),
        )?;
        let records = [common::record(1), common::record(2)];
        if std::env::var_os("EVE_STORAGE_TEST_STAGE_ONLY").is_none() {
            commit_record_batch(&mut store, &records)?;
        }
        // Exit does not run Rust destructors; this is process-exit, not power-loss evidence.
        std::process::exit(86);
    }
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("database");
    drop(open_record_store(
        &database_path,
        common::identity(),
        common::budget(),
    )?);
    for staged in [true, false] {
        let mut command = std::process::Command::new(std::env::current_exe()?);
        command.args([
            "--exact",
            "synced_batch_survives_process_exit_without_database_destructors",
            "--nocapture",
        ]);
        command.env("EVE_STORAGE_TEST_CHILD_PATH", &database_path);
        if staged {
            command.env("EVE_STORAGE_TEST_STAGE_ONLY", "1");
        }
        let status = command.status()?;
        assert_eq!(status.code(), Some(86));
        let reopened = open_record_store(&database_path, common::identity(), common::budget())?;
        assert_eq!(
            read_record_cursor(&reopened)?.height,
            if staged { 0 } else { 2 }
        );
        assert_eq!(
            read_record(&reopened, 1)?,
            if staged {
                None
            } else {
                Some(common::record(1))
            }
        );
        assert_eq!(
            read_record(&reopened, 2)?,
            if staged {
                None
            } else {
                Some(common::record(2))
            }
        );
    }
    Ok(())
}
