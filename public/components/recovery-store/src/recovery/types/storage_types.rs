use rocksdb::{DB, SnapshotWithThreadMode};
use serde::Serialize;
use std::sync::Arc;

/// An untrusted storage namespace identifier, not a consensus identity proof.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct StorageNetworkId(pub [u8; 32]);

/// Caller-provided storage bootstrap metadata. Authentication belongs upstream.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct StorageIdentity {
    pub network: StorageNetworkId,
    pub genesis_hash: [u8; 32],
    pub base_height: u64,
    pub base_block_hash: [u8; 32],
}

/// Opaque already-serialized execution/recovery input; this type is not verified.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StoredBlockInput {
    pub identity: StorageIdentity,
    pub parent_height: u64,
    pub height: u64,
    pub parent_block_hash: [u8; 32],
    pub block_hash: [u8; 32],
    pub payload: Vec<u8>,
}

/// Locally synced record prefix; not finality or authenticated state height.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct DurableRecordCursor {
    pub identity: StorageIdentity,
    pub height: u64,
    pub block_hash: [u8; 32],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct StorageBudget {
    pub max_record_bytes: usize,
    pub max_batch_bytes: usize,
    pub max_batch_records: usize,
    pub write_buffer_bytes: usize,
    pub write_buffer_count: i32,
    pub block_cache_bytes: usize,
    pub max_background_jobs: i32,
    pub max_open_files: i32,
}

/// One local namespace owner. No raw database handle is exposed to consumers.
pub struct DurableRecordStore {
    pub(crate) database: Arc<DB>,
    pub(crate) identity: StorageIdentity,
    pub(crate) budget: StorageBudget,
}

/// A read-only capability that can outlive a borrow of the single writer.
pub struct RecordReader {
    pub(crate) database: Arc<DB>,
    pub(crate) identity: StorageIdentity,
    pub(crate) budget: StorageBudget,
}

pub struct RecordSnapshot<'a> {
    pub(crate) snapshot: SnapshotWithThreadMode<'a, DB>,
    pub(crate) identity: StorageIdentity,
    pub(crate) budget: StorageBudget,
    pub cursor: DurableRecordCursor,
}

pub(crate) const IDENTITY_KEY: &[u8] = b"eve/storage/v1/identity";
pub(crate) const CURSOR_KEY: &[u8] = b"eve/storage/v1/durable-record-cursor";
pub(crate) const RECORD_PREFIX: &[u8] = b"eve/storage/v1/record/";
pub(crate) const MAGIC: &[u8; 8] = b"EVESTR01";
pub(crate) const IDENTITY_BYTES: usize = 112;
pub(crate) const RECORD_HEADER_BYTES: usize = 200;
pub(crate) const CURSOR_BYTES: usize = 152;
