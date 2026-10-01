// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};

use crate::recovery::records::read_record;
use crate::recovery::types::{DurableRecordCursor, DurableRecordStore, StoredBlockInput};
use crate::recovery::validation::validate_record_sequence::validate_record_sequence;

pub fn check_replayed_prefix(
    store: &DurableRecordStore,
    records: &[StoredBlockInput],
    cursor: &DurableRecordCursor,
) -> Result<usize> {
    let first = records.first().context("empty replay batch")?;
    let initial = DurableRecordCursor {
        identity: store.identity,
        height: first.parent_height,
        block_hash: first.parent_block_hash,
    };
    validate_record_sequence(&initial, records)?;
    let mut count = 0;
    for record in records
        .iter()
        .take_while(|record| record.height <= cursor.height)
    {
        let existing = read_record(store, record.height)?
            .context("replay references missing durable record")?;
        ensure!(existing == *record, "replay differs from durable record");
        count += 1;
    }
    Ok(count)
}
