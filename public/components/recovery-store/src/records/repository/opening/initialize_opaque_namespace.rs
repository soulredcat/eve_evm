// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{
    OpaqueRecordBudget, OpaqueRecordCursor, OpaqueRecordIdentity,
    encoding::{encode_opaque_cursor, encode_opaque_identity},
    types::schema::{HEAD_KEY, IDENTITY_KEY, SCHEMA, SCHEMA_KEY},
};
use anyhow::{Result, ensure};
use rocksdb::{DB, WriteBatch, WriteOptions};

pub(in crate::records) fn initialize_opaque_namespace(
    database: &DB,
    identity: OpaqueRecordIdentity,
    head: OpaqueRecordCursor,
    budget: &OpaqueRecordBudget,
) -> Result<()> {
    ensure!(
        database.latest_sequence_number() == 0,
        "new opaque namespace is unexpectedly nonempty"
    );
    let mut batch = WriteBatch::default();
    batch.put(SCHEMA_KEY, SCHEMA);
    batch.put(IDENTITY_KEY, encode_opaque_identity(identity));
    batch.put(HEAD_KEY, encode_opaque_cursor(head));
    ensure!(
        batch.size_in_bytes() <= budget.maximum_batch_bytes,
        "opaque initialization metadata exceeds batch limit"
    );
    let mut writes = WriteOptions::default();
    writes.set_sync(true);
    writes.disable_wal(false);
    database.write_opt(batch, &writes)?;
    Ok(())
}
