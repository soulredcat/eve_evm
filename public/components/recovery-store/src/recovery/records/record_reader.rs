use std::sync::Arc;

use crate::recovery::types::{DurableRecordStore, RecordReader};

pub fn record_reader(store: &DurableRecordStore) -> RecordReader {
    RecordReader {
        database: Arc::clone(&store.database),
        identity: store.identity,
        budget: store.budget,
    }
}
