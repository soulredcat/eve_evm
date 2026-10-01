// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Context, Result, ensure};

use crate::recovery::encoding::cursors::decode_record_cursor::decode_record_cursor;
use crate::recovery::types::{CURSOR_KEY, RecordReader, RecordSnapshot};

pub fn capture_record_snapshot(reader: &RecordReader) -> Result<RecordSnapshot<'_>> {
    let snapshot = reader.database.snapshot();
    let cursor = decode_record_cursor(
        &snapshot
            .get(CURSOR_KEY)?
            .context("snapshot missing durable cursor")?,
    )?;
    ensure!(
        cursor.identity == reader.identity,
        "snapshot cursor identity mismatch"
    );
    Ok(RecordSnapshot {
        snapshot,
        identity: reader.identity,
        budget: reader.budget,
        cursor,
    })
}
