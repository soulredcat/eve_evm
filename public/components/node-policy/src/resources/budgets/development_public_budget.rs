// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::PublicBudget;

/// Cache/write-buffer/job/file knobs reproduce the B0 RocksDB API spike.
/// Queue, process, lag and routing ceilings are conservative development policy;
/// B4/B6 must measure/enforce them, not treat this factory as runtime acceptance.
pub fn development_public_budget() -> PublicBudget {
    let mib = 1_048_576;
    PublicBudget {
        version: 1,
        maximum_record_bytes: 8 * mib,
        maximum_batch_bytes: 8 * mib,
        maximum_batch_records: 1,
        queue_bytes: 32 * mib,
        queue_batches: 4,
        queue_age_ms: 2_000,
        maximum_durable_lag_blocks: 8,
        maximum_authenticated_lag_blocks: 1,
        maximum_head_lag_blocks: 2,
        write_buffer_bytes: 4 * mib,
        write_buffer_count: 2,
        block_cache_bytes: 4 * mib,
        background_jobs: 2,
        open_files: 32,
        worker_threads: 1,
        worker_cpu_basis_points: 2_500,
        concurrent_storage_reads: 2,
        maximum_working_state_bytes: 256 * mib,
        query_cache_bytes: 16 * mib,
        mempool_bytes: 16 * mib,
        simulation_overlay_bytes: 16 * mib,
        snapshot_staging_bytes: 16 * mib,
        process_memory_budget_bytes: 512 * mib,
        active_peer_connections: 64,
        concurrent_bulk_requests: 8,
        per_peer_inflight_bytes: 32 * mib,
        global_bulk_inflight_bytes: 64 * mib,
        ordinary_envelope_bytes: mib,
        global_ordinary_inflight_bytes: 16 * mib,
        snapshot_chunk_bytes: 4 * mib,
        decompressed_chunk_bytes: 8 * mib,
        probe_candidates: 32,
        concurrent_probes: 4,
        probe_deadline_ms: 2_000,
        retry_maximum_ms: 30_000,
        switch_cooldown_ms: 30_000,
        switch_improvement_basis_points: 1_500,
        maximum_source_error_basis_points: 1_000,
    }
}
