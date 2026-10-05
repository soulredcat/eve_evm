// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::RecordWorkerError;
use eve_node_policy::{PublicBudget, validate_public_budget};
use eve_storage::records::{OpaqueRecordBudget, validate_opaque_record_budget};

/// Admit additional staging only inside the existing profile's unallocated logical allowance.
pub(super) fn validate_worker_resources(
    public: PublicBudget,
    repository: OpaqueRecordBudget,
    scratch_limit: u64,
) -> Result<(), RecordWorkerError> {
    let invalid = RecordWorkerError::InvalidConfiguration;
    validate_public_budget(public).map_err(|_| invalid)?;
    validate_opaque_record_budget(&repository).map_err(|_| invalid)?;
    if scratch_limit == 0
        || repository.block_cache_bytes as u64 > public.block_cache_bytes
        || repository.write_buffer_bytes as u64 > public.write_buffer_bytes
        || repository.write_buffer_count as u64 > public.write_buffer_count
        || repository.maximum_background_jobs as u64 > public.background_jobs
        || repository.maximum_open_files as u64 > public.open_files
    {
        return Err(invalid);
    }
    // Mirror the validated v1 profile's declared pools, without treating them as measured RSS.
    let write_buffers = public
        .write_buffer_bytes
        .checked_mul(public.write_buffer_count)
        .ok_or(invalid)?;
    let configured = [
        write_buffers,
        public.block_cache_bytes,
        public.maximum_batch_bytes,
        public.queue_bytes,
        public.maximum_working_state_bytes,
        public.query_cache_bytes,
        public.mempool_bytes,
        public.simulation_overlay_bytes,
        public.snapshot_staging_bytes,
        public.global_bulk_inflight_bytes,
        public.global_ordinary_inflight_bytes,
    ]
    .into_iter()
    .try_fold(0_u64, |sum, bytes| sum.checked_add(bytes))
    .ok_or(invalid)?;
    let available = public
        .process_memory_budget_bytes
        .checked_sub(configured)
        .ok_or(invalid)?;
    if scratch_limit > available {
        return Err(invalid);
    }
    Ok(())
}
