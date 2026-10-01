// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod common;

use anyhow::Result;
use eve_storage::recovery::opening::open_record_store;
use eve_storage::recovery::records::{commit_record_batch, read_record, read_record_cursor};

#[test]
fn synced_batch_reopens_with_exact_metadata_and_payload() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let path = directory.path().join("database");
    let mut store = open_record_store(&path, common::identity(), common::budget())?;
    let records = [common::record(1), common::record(2)];
    let cursor = commit_record_batch(&mut store, &records)?;
    assert_eq!(cursor.height, 2);
    assert_eq!(cursor.block_hash, [2; 32]);
    drop(store);
    let reopened = open_record_store(&path, common::identity(), common::budget())?;
    assert_eq!(read_record_cursor(&reopened)?, cursor);
    assert_eq!(read_record(&reopened, 1)?, Some(records[0].clone()));
    assert_eq!(read_record(&reopened, 2)?, Some(records[1].clone()));
    assert_eq!(read_record(&reopened, 3)?, None);
    Ok(())
}

#[test]
fn replay_is_idempotent_and_changed_replay_is_rejected() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let mut store = open_record_store(directory.path(), common::identity(), common::budget())?;
    let one = common::record(1);
    let first = commit_record_batch(&mut store, std::slice::from_ref(&one))?;
    assert_eq!(
        commit_record_batch(&mut store, std::slice::from_ref(&one))?,
        first
    );
    let two = common::record(2);
    assert_eq!(
        commit_record_batch(&mut store, &[one.clone(), two.clone()])?.height,
        2
    );
    let mut changed = one;
    changed.payload.push(0xff);
    assert!(commit_record_batch(&mut store, &[changed]).is_err());
    assert_eq!(read_record(&store, 2)?, Some(two));
    assert_eq!(read_record_cursor(&store)?.height, 2);
    Ok(())
}

#[test]
fn mismatched_identity_or_concurrent_namespace_owner_is_rejected() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let store = open_record_store(directory.path(), common::identity(), common::budget())?;
    assert!(open_record_store(directory.path(), common::identity(), common::budget()).is_err());
    drop(store);
    let mut other = common::identity();
    other.genesis_hash = [9; 32];
    assert!(open_record_store(directory.path(), other, common::budget()).is_err());
    other = common::identity();
    other.network.0 = [9; 32];
    assert!(open_record_store(directory.path(), other, common::budget()).is_err());
    assert_eq!(
        read_record_cursor(&open_record_store(
            directory.path(),
            common::identity(),
            common::budget()
        )?)?
        .height,
        0
    );
    Ok(())
}

#[test]
fn bad_identity_parent_height_and_missing_payload_never_advance_cursor() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let mut store = open_record_store(directory.path(), common::identity(), common::budget())?;
    let mut cases = vec![common::record(1); 4];
    cases[0].identity.genesis_hash = [9; 32];
    cases[1].parent_block_hash = [9; 32];
    cases[2].height = 3;
    cases[3].payload.clear();
    for bad in cases {
        assert!(commit_record_batch(&mut store, &[bad]).is_err());
        assert_eq!(read_record_cursor(&store)?.height, 0);
        assert!(read_record(&store, 1)?.is_none());
    }
    Ok(())
}

#[test]
fn record_batch_and_empty_limits_fail_without_partial_height() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let mut budget = common::budget();
    budget.max_record_bytes = 512;
    budget.max_batch_bytes = 1024;
    budget.max_batch_records = 2;
    let mut store = open_record_store(directory.path(), common::identity(), budget)?;
    assert!(commit_record_batch(&mut store, &[]).is_err());
    assert!(
        commit_record_batch(
            &mut store,
            &[common::record(1), common::record(2), common::record(3)]
        )
        .is_err()
    );
    let mut oversized = common::record(1);
    oversized.payload.resize(313, 0);
    assert!(commit_record_batch(&mut store, &[oversized]).is_err());
    let mut full = [common::record(1), common::record(2)];
    full[0].payload.resize(312, 0);
    full[1].payload.resize(312, 0);
    assert!(commit_record_batch(&mut store, &full).is_err());
    assert_eq!(read_record_cursor(&store)?.height, 0);
    assert!(read_record(&store, 1)?.is_none());
    Ok(())
}
