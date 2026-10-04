// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::archive_types::ArchiveMaintenance;
use crate::sync::applied::tests::import_fixtures::ImportChain;
use eve_storage::records::{
    OpaqueCompactionRequest, compact_opaque_records, opaque_record_cursor,
    open_opaque_record_repository, read_opaque_record, required_opaque_compaction_reservation,
};

/// Real KEEP_ALL maintenance and exact retained payload/cursor checks, not reclaimed-byte claims.
pub(super) fn compact_retained_archive(
    archive: &ArchiveMaintenance,
    chain: &ImportChain,
) -> Result<(), String> {
    let mut repository = open_opaque_record_repository(
        &archive.opaque_path,
        archive.identity,
        archive.opaque_budget,
    )
    .map_err(|_| "maintenance opaque reopen")?;
    let required = required_opaque_compaction_reservation(&archive.opaque_budget)
        .map_err(|_| "maintenance compaction reservation")?;
    let head = compact_opaque_records(
        &mut repository,
        &OpaqueCompactionRequest {
            expected_identity: archive.identity,
            expected_head: archive.head,
            maximum_records: archive.opaque_budget.maximum_retained_records,
        },
        required,
    )
    .map_err(|_| "actual KEEP_ALL compaction")?;
    if head != archive.head
        || opaque_record_cursor(&repository).map_err(|_| "maintenance opaque head")? != archive.head
    {
        return Err("compaction changed actual retained cursor".into());
    }
    for (index, expected) in chain.records.iter().enumerate() {
        let row = read_opaque_record(&repository, index as u64 + 1)
            .map_err(|_| "maintenance retained read")?
            .ok_or("maintenance retained row missing")?;
        if row.payload != *expected {
            return Err("compaction changed exact retained payload".into());
        }
    }
    if read_opaque_record(&repository, chain.records.len() as u64 + 1)
        .map_err(|_| "maintenance boundary read")?
        .is_some()
    {
        return Err("compaction invented a retained row".into());
    }
    Ok(())
}
