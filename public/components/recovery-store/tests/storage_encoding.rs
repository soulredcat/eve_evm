mod common;

use anyhow::Result;
use eve_storage::recovery::opening::open_record_store;
use eve_storage::recovery::records::{commit_record_batch, read_record, read_record_cursor};
use rocksdb::DB;

#[test]
fn fixed_width_storage_bytes_match_independent_golden_layout() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let mut store = open_record_store(directory.path(), common::identity(), common::budget())?;
    commit_record_batch(&mut store, &[common::record(1)])?;
    drop(store);
    let raw = DB::open_default(directory.path())?;
    let key = [b"eve/storage/v1/record/".as_slice(), &1_u64.to_be_bytes()].concat();
    let bytes = raw.get(key)?.expect("stored record");
    let mut golden = b"EVESTR01".to_vec();
    golden.extend_from_slice(&[1; 32]);
    golden.extend_from_slice(&[2; 32]);
    golden.extend_from_slice(&0_u64.to_be_bytes());
    golden.extend_from_slice(&[3; 32]);
    golden.extend_from_slice(&0_u64.to_be_bytes());
    golden.extend_from_slice(&1_u64.to_be_bytes());
    golden.extend_from_slice(&[3; 32]);
    golden.extend_from_slice(&[1; 32]);
    golden.extend_from_slice(&2_u64.to_be_bytes());
    golden.extend_from_slice(&[0xa5, 0x5a]);
    assert_eq!(bytes, golden);
    assert_eq!(bytes.len(), 202);
    Ok(())
}

#[test]
fn truncated_noncanonical_and_missing_tail_records_are_rejected() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let mut store = open_record_store(directory.path(), common::identity(), common::budget())?;
    commit_record_batch(&mut store, &[common::record(1), common::record(2)])?;
    drop(store);
    let key_one = [b"eve/storage/v1/record/".as_slice(), &1_u64.to_be_bytes()].concat();
    let key_two = [b"eve/storage/v1/record/".as_slice(), &2_u64.to_be_bytes()].concat();
    {
        let raw = DB::open_default(directory.path())?;
        let mut corrupt = raw.get(&key_one)?.expect("stored record");
        corrupt.push(0);
        raw.put(&key_one, corrupt)?;
        raw.flush_wal(true)?;
    }
    let store = open_record_store(directory.path(), common::identity(), common::budget())?;
    assert!(read_record(&store, 1).is_err());
    assert_eq!(read_record_cursor(&store)?.height, 2);
    drop(store);
    {
        let raw = DB::open_default(directory.path())?;
        raw.put(&key_two, b"EVESTR01")?;
        raw.flush_wal(true)?;
    }
    assert!(open_record_store(directory.path(), common::identity(), common::budget()).is_err());
    {
        let raw = DB::open_default(directory.path())?;
        raw.delete(key_two)?;
        raw.flush_wal(true)?;
    }
    assert!(open_record_store(directory.path(), common::identity(), common::budget()).is_err());
    Ok(())
}
