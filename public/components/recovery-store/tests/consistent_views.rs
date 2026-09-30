mod common;

use anyhow::Result;
use eve_storage::recovery::checkpoint::create_record_checkpoint;
use eve_storage::recovery::opening::open_record_store;
use eve_storage::recovery::records::{
    commit_record_batch, read_record, read_record_cursor, record_reader,
};
use eve_storage::recovery::snapshot::{capture_record_snapshot, read_snapshot_record};

#[test]
fn snapshot_preserves_height_and_payload_across_new_commit() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let mut store = open_record_store(directory.path(), common::identity(), common::budget())?;
    commit_record_batch(&mut store, &[common::record(1)])?;
    let reader = record_reader(&store);
    let snapshot = capture_record_snapshot(&reader)?;
    commit_record_batch(&mut store, &[common::record(2)])?;
    assert_eq!(snapshot.cursor.height, 1);
    assert_eq!(read_snapshot_record(&snapshot, 1)?, Some(common::record(1)));
    assert!(read_snapshot_record(&snapshot, 2)?.is_none());
    assert_eq!(read_record_cursor(&store)?.height, 2);
    assert_eq!(read_record(&store, 2)?, Some(common::record(2)));
    Ok(())
}

#[test]
fn checkpoint_reopens_at_capture_height_and_destination_is_not_overwritten() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let database_path = directory.path().join("database");
    let checkpoint_path = directory.path().join("checkpoint");
    let mut store = open_record_store(&database_path, common::identity(), common::budget())?;
    commit_record_batch(&mut store, &[common::record(1)])?;
    assert_eq!(
        create_record_checkpoint(&mut store, &checkpoint_path)?.height,
        1
    );
    commit_record_batch(&mut store, &[common::record(2)])?;
    assert!(create_record_checkpoint(&mut store, &checkpoint_path).is_err());
    let checkpoint = open_record_store(&checkpoint_path, common::identity(), common::budget())?;
    assert_eq!(read_record_cursor(&checkpoint)?.height, 1);
    assert_eq!(read_record(&checkpoint, 1)?, Some(common::record(1)));
    assert!(read_record(&checkpoint, 2)?.is_none());
    Ok(())
}
