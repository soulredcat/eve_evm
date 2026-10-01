// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

/// Knobs/limits, not proof of OS enforcement or zero storage interference.
/// Global admission must reserve bytes before allocating ordinary/bulk payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PublicBudget {
    pub version: u32,
    pub maximum_record_bytes: u64,
    pub maximum_batch_bytes: u64,
    pub maximum_batch_records: u64,
    pub queue_bytes: u64,
    pub queue_batches: u64,
    pub queue_age_ms: u64,
    pub maximum_durable_lag_blocks: u64,
    pub maximum_authenticated_lag_blocks: u64,
    pub maximum_head_lag_blocks: u64,
    pub write_buffer_bytes: u64,
    pub write_buffer_count: u64,
    pub block_cache_bytes: u64,
    pub background_jobs: u64,
    pub open_files: u64,
    pub worker_threads: u64,
    pub worker_cpu_basis_points: u64,
    pub concurrent_storage_reads: u64,
    pub maximum_working_state_bytes: u64,
    pub query_cache_bytes: u64,
    pub mempool_bytes: u64,
    pub simulation_overlay_bytes: u64,
    pub snapshot_staging_bytes: u64,
    pub process_memory_budget_bytes: u64,
    pub active_peer_connections: u64,
    pub concurrent_bulk_requests: u64,
    pub per_peer_inflight_bytes: u64,
    pub global_bulk_inflight_bytes: u64,
    pub ordinary_envelope_bytes: u64,
    pub global_ordinary_inflight_bytes: u64,
    pub snapshot_chunk_bytes: u64,
    pub decompressed_chunk_bytes: u64,
    pub probe_candidates: u64,
    pub concurrent_probes: u64,
    pub probe_deadline_ms: u64,
    pub retry_maximum_ms: u64,
    pub switch_cooldown_ms: u64,
    pub switch_improvement_basis_points: u64,
    pub maximum_source_error_basis_points: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BudgetError {
    UnsupportedVersion,
    InvalidLimit,
    InconsistentBound,
    ArithmeticOverflow,
}
