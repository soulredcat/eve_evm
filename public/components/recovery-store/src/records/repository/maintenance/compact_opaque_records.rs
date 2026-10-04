// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    OpaqueCompactionRequest, required_opaque_compaction_reservation,
    verify_opaque_compaction_head::verify_opaque_compaction_head,
};
use crate::records::{OpaqueRecordCursor, OpaqueRecordRepository, opaque_record_cursor};
use anyhow::{Result, bail, ensure};
use rocksdb::FlushOptions;
use std::sync::atomic::Ordering;

/// Exclusive-owner KEEP_ALL maintenance. No rows are deleted and no new append
/// or finality acknowledgement is produced. Schedule separately from RAM work.
pub fn compact_opaque_records(
    repository: &mut OpaqueRecordRepository,
    request: &OpaqueCompactionRequest,
    reserved_read_bytes: usize,
) -> Result<OpaqueRecordCursor> {
    let head = opaque_record_cursor(repository)?;
    ensure!(
        request.expected_identity == repository.identity,
        "opaque compaction namespace mismatch"
    );
    ensure!(
        request.expected_head == head,
        "opaque compaction expected head mismatch"
    );
    ensure!(
        request.maximum_records > 0
            && request.maximum_records <= repository.budget.maximum_retained_records
            && head.sequence <= request.maximum_records,
        "opaque compaction record admission limit"
    );
    ensure!(
        reserved_read_bytes >= required_opaque_compaction_reservation(&repository.budget)?,
        "opaque compaction read reservation missing"
    );
    if let Err(error) = verify_opaque_compaction_head(repository, head) {
        repository.fenced.store(true, Ordering::Release);
        bail!("opaque compaction preflight failed; handle fenced: {error}");
    }
    if let Err(error) = repository.database.flush_wal(true) {
        repository.fenced.store(true, Ordering::Release);
        bail!("opaque compaction WAL sync failed; handle fenced: {error}");
    }
    let mut flush = FlushOptions::default();
    flush.set_wait(true);
    if let Err(error) = repository.database.flush_opt(&flush) {
        repository.fenced.store(true, Ordering::Release);
        bail!("opaque compaction memtable flush failed; handle fenced: {error}");
    }
    // The pinned safe API returns no compaction status. Revalidate all retained
    // records after the call; do not infer new durability/finality from completion.
    repository
        .database
        .compact_range::<&[u8], &[u8]>(None, None);
    if let Err(error) = verify_opaque_compaction_head(repository, head) {
        repository.fenced.store(true, Ordering::Release);
        bail!("opaque compaction retained-data validation failed; handle fenced: {error}");
    }
    Ok(head)
}
