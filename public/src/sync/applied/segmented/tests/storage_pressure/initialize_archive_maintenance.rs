// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{archive_types::ArchiveMaintenance, types::MeasurementContract};
use crate::sync::applied::{
    AppliedReader, reserve_applied_snapshot_staging, reserve_applied_storage_read,
    reserve_applied_working, tests::import_fixtures::ImportChain,
};
use eve_storage::{
    records::{
        OpaqueRecordIdentity, compare_and_append_opaque_records, development_opaque_record_budget,
        opaque_record_cursor, open_opaque_record_repository,
    },
    state::{
        HistoryReadBudget, commit_state, development_state_storage_budget, ensure_history_index,
        open_state_repository,
    },
};
use std::os::unix::fs::PermissionsExt;

/// Only one auxiliary database is open at a time; both contain real unchanged canonical input.
pub(super) fn initialize_archive_maintenance(
    reader: &AppliedReader,
    chain: &ImportChain,
    contract: &MeasurementContract,
) -> Result<ArchiveMaintenance, String> {
    let _read = reserve_applied_storage_read(reader).map_err(|_| "archive read admission")?;
    let _working =
        reserve_applied_working(reader, contract.auxiliary_job_working_reservation_bytes)
            .map_err(|_| "archive working admission")?;
    let _raw =
        reserve_applied_snapshot_staging(reader, contract.auxiliary_job_staging_reservation_bytes)
            .map_err(|_| "archive raw staging admission")?;
    let directory = tempfile::Builder::new()
        .permissions(std::fs::Permissions::from_mode(0o700))
        .tempdir()
        .map_err(|_| "archive temporary namespace")?;
    let opaque_path = directory.path().join("opaque");
    let index_path = directory.path().join("state-index");
    let identity = OpaqueRecordIdentity {
        genesis_hash: chain.commits[0].target.identity.genesis.0.0,
        owner: [0x71; 32],
        domain: [0x72; 32],
    };
    let mut opaque_budget = development_opaque_record_budget();
    opaque_budget.maximum_record_bytes = 65_536;
    opaque_budget.maximum_read_bytes = 65_536;
    opaque_budget.maximum_batch_bytes = 131_072;
    opaque_budget.maximum_batch_records = 1;
    opaque_budget.maximum_retained_records = chain.records.len() as u64;
    opaque_budget.block_cache_bytes = contract.auxiliary_database_block_cache_bytes;
    opaque_budget.write_buffer_bytes = contract.auxiliary_database_write_buffer_bytes;
    opaque_budget.write_buffer_count = contract.auxiliary_database_write_buffer_count;
    opaque_budget.maximum_background_jobs = contract.auxiliary_database_background_jobs;
    opaque_budget.maximum_open_files = contract.auxiliary_database_open_files;
    let head = {
        let mut repository = open_opaque_record_repository(&opaque_path, identity, opaque_budget)
            .map_err(|_| "archive opaque open")?;
        for bytes in &chain.records {
            let previous = opaque_record_cursor(&repository).map_err(|_| "archive opaque head")?;
            compare_and_append_opaque_records(
                &mut repository,
                previous,
                std::slice::from_ref(bytes),
            )
            .map_err(|_| "archive canonical payload sync")?;
        }
        opaque_record_cursor(&repository).map_err(|_| "archive opaque head")?
    };
    let mut state_budget = development_state_storage_budget();
    state_budget.database.max_record_bytes = 65_536;
    state_budget.database.max_batch_bytes = 131_072;
    state_budget.database.max_batch_records = 1;
    state_budget.database.block_cache_bytes = contract.auxiliary_database_block_cache_bytes;
    state_budget.database.write_buffer_bytes = contract.auxiliary_database_write_buffer_bytes;
    state_budget.database.write_buffer_count = contract.auxiliary_database_write_buffer_count;
    state_budget.database.max_background_jobs = contract.auxiliary_database_background_jobs;
    state_budget.database.max_open_files = contract.auxiliary_database_open_files;
    state_budget.maximum_commit_bytes = 65_536;
    state_budget.maximum_snapshots = 1;
    let history_budget = HistoryReadBudget {
        maximum_block_bytes: 65_536,
        maximum_rebuild_blocks: chain.commits.len(),
        maximum_index_batch_bytes: 65_536,
    };
    {
        let mut repository = open_state_repository(&index_path, &chain.commits[0], state_budget)
            .map_err(|_| "archive state open")?;
        for commit in &chain.commits[1..] {
            commit_state(&mut repository, commit).map_err(|_| "archive canonical state sync")?;
        }
        let status = ensure_history_index(&mut repository, history_budget)
            .map_err(|_| "archive canonical index")?;
        if !status.complete {
            return Err("archive canonical index incomplete".into());
        }
    }
    Ok(ArchiveMaintenance {
        _directory: directory,
        opaque_path,
        index_path,
        opaque_budget,
        state_budget,
        history_budget,
        identity,
        head,
    })
}
