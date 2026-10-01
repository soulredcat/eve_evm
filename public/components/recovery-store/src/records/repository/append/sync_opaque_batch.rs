// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::OpaqueRecordRepository;
use anyhow::{Result, bail};
use rocksdb::{WriteBatch, WriteOptions};
use std::sync::atomic::Ordering;

pub(in crate::records) fn sync_opaque_batch(
    store: &mut OpaqueRecordRepository,
    batch: WriteBatch,
) -> Result<()> {
    let mut writes = WriteOptions::default();
    writes.set_sync(true);
    writes.disable_wal(false);
    #[cfg(test)]
    if matches!(
        store.simulated_failure,
        Some(crate::records::types::SimulatedOpaqueFailure::BeforeWrite)
    ) {
        store.simulated_failure = None;
        store.fenced.store(true, Ordering::Release);
        bail!("SIMULATED pre-write ambiguous outcome; opaque handle fenced");
    }
    if let Err(error) = store.database.write_opt(batch, &writes) {
        store.fenced.store(true, Ordering::Release);
        bail!("ambiguous opaque write/sync outcome; handle fenced: {error}");
    }
    #[cfg(test)]
    if matches!(
        store.simulated_failure,
        Some(crate::records::types::SimulatedOpaqueFailure::AfterSuccessfulSync)
    ) {
        store.simulated_failure = None;
        store.fenced.store(true, Ordering::Release);
        bail!("SIMULATED lost acknowledgment after actual successful opaque sync");
    }
    Ok(())
}
