// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{
    OpaqueRecordBudget, OpaqueRecordCursor, OpaqueRecordIdentity,
    encoding::{
        decode_opaque_cursor, decode_opaque_record, encode_opaque_identity, opaque_record_key,
    },
    hashing::hash_opaque_record,
    types::schema::{HEAD_KEY, IDENTITY_KEY, RECORD_PREFIX, SCHEMA, SCHEMA_KEY},
};
use anyhow::{Context, Result, ensure};
use rocksdb::DB;

/// Validate every retained row with bounded decoded allocations; no orphan or unknown keys are accepted.
pub(in crate::records) fn validate_opaque_namespace(
    database: &DB,
    identity: OpaqueRecordIdentity,
    bootstrap: OpaqueRecordCursor,
    budget: &OpaqueRecordBudget,
) -> Result<OpaqueRecordCursor> {
    ensure!(
        database.get_pinned(SCHEMA_KEY)?.as_deref() == Some(SCHEMA),
        "missing/unsupported opaque schema; namespace cannot reset"
    );
    ensure!(
        database.get_pinned(IDENTITY_KEY)?.as_deref()
            == Some(encode_opaque_identity(identity).as_slice()),
        "opaque immutable identity/domain mismatch"
    );
    let head = decode_opaque_cursor(
        &database
            .get_pinned(HEAD_KEY)?
            .context("opaque durable head missing")?,
    )?;
    ensure!(
        head.sequence <= budget.maximum_retained_records,
        "opaque history exceeds retained-count limit"
    );
    let mut current = bootstrap;
    let mut metadata_count = 0;
    let mut iterator = database.raw_iterator();
    iterator.seek_to_first();
    while iterator.valid() {
        let key = iterator.key().context("opaque iterator key missing")?;
        if [SCHEMA_KEY, IDENTITY_KEY, HEAD_KEY].contains(&key) {
            metadata_count += 1;
        } else {
            ensure!(
                key.starts_with(RECORD_PREFIX),
                "foreign key in dedicated opaque namespace"
            );
            let sequence = current
                .sequence
                .checked_add(1)
                .context("opaque sequence overflow")?;
            ensure!(
                sequence <= head.sequence && key == opaque_record_key(sequence),
                "orphan/gapped opaque history key"
            );
            let record = decode_opaque_record(
                iterator.value().context("opaque iterator value missing")?,
                budget,
            )?;
            ensure!(
                record.sequence == sequence && record.parent == current,
                "opaque record sequence/parent mismatch"
            );
            ensure!(
                record.content_hash
                    == hash_opaque_record(identity, current, sequence, &record.payload),
                "opaque record content binding mismatch"
            );
            current = OpaqueRecordCursor {
                sequence,
                content_hash: record.content_hash,
            };
        }
        iterator.next();
    }
    iterator.status()?;
    ensure!(
        metadata_count == 3 && current == head,
        "opaque retained history/head incomplete or inconsistent"
    );
    Ok(head)
}
