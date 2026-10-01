// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{BudgetError, PublicBudget};

pub fn validate_public_budget(value: PublicBudget) -> Result<(), BudgetError> {
    if value.version != 1 {
        return Err(BudgetError::UnsupportedVersion);
    }
    let positive = [
        value.maximum_record_bytes,
        value.maximum_batch_bytes,
        value.maximum_batch_records,
        value.queue_bytes,
        value.queue_batches,
        value.queue_age_ms,
        value.maximum_durable_lag_blocks,
        value.write_buffer_bytes,
        value.write_buffer_count,
        value.block_cache_bytes,
        value.background_jobs,
        value.open_files,
        value.worker_threads,
        value.worker_cpu_basis_points,
        value.concurrent_storage_reads,
        value.maximum_working_state_bytes,
        value.query_cache_bytes,
        value.mempool_bytes,
        value.simulation_overlay_bytes,
        value.snapshot_staging_bytes,
        value.process_memory_budget_bytes,
        value.active_peer_connections,
        value.concurrent_bulk_requests,
        value.per_peer_inflight_bytes,
        value.global_bulk_inflight_bytes,
        value.ordinary_envelope_bytes,
        value.global_ordinary_inflight_bytes,
        value.snapshot_chunk_bytes,
        value.decompressed_chunk_bytes,
        value.probe_candidates,
        value.concurrent_probes,
        value.probe_deadline_ms,
        value.retry_maximum_ms,
        value.switch_cooldown_ms,
    ];
    if positive.contains(&0)
        || value.worker_cpu_basis_points > 10_000
        || value.switch_improvement_basis_points > 10_000
        || value.maximum_source_error_basis_points > 10_000
    {
        return Err(BudgetError::InvalidLimit);
    }
    if value.maximum_record_bytes > value.maximum_batch_bytes
        || value.maximum_batch_bytes > value.queue_bytes
        || value.snapshot_chunk_bytes > value.decompressed_chunk_bytes
        || value.decompressed_chunk_bytes > value.per_peer_inflight_bytes
        || value.ordinary_envelope_bytes > value.per_peer_inflight_bytes
        || value.per_peer_inflight_bytes > value.global_bulk_inflight_bytes
        || value.ordinary_envelope_bytes > value.global_ordinary_inflight_bytes
        || value.concurrent_probes > value.probe_candidates
        || value.probe_candidates > value.active_peer_connections
        || value.concurrent_bulk_requests > value.active_peer_connections
        || value.retry_maximum_ms < value.probe_deadline_ms
    {
        return Err(BudgetError::InconsistentBound);
    }
    let queued_capacity = value
        .maximum_batch_bytes
        .checked_mul(value.queue_batches)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    if queued_capacity > value.queue_bytes {
        return Err(BudgetError::InconsistentBound);
    }
    let buffers = value
        .write_buffer_bytes
        .checked_mul(value.write_buffer_count)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let bulk_capacity = value
        .per_peer_inflight_bytes
        .checked_mul(value.concurrent_bulk_requests)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    let ordinary_capacity = value
        .ordinary_envelope_bytes
        .checked_mul(value.active_peer_connections)
        .ok_or(BudgetError::ArithmeticOverflow)?;
    if value.global_bulk_inflight_bytes > bulk_capacity
        || value.global_ordinary_inflight_bytes > ordinary_capacity
    {
        return Err(BudgetError::InconsistentBound);
    }
    let memory = [
        buffers,
        value.block_cache_bytes,
        value.maximum_batch_bytes,
        value.queue_bytes,
        value.maximum_working_state_bytes,
        value.query_cache_bytes,
        value.mempool_bytes,
        value.simulation_overlay_bytes,
        value.snapshot_staging_bytes,
        value.global_bulk_inflight_bytes,
        value.global_ordinary_inflight_bytes,
    ]
    .into_iter()
    .try_fold(0_u64, |sum, bytes| sum.checked_add(bytes))
    .ok_or(BudgetError::ArithmeticOverflow)?;
    if memory >= value.process_memory_budget_bytes {
        return Err(BudgetError::InconsistentBound);
    }
    Ok(())
}
