// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::MeasurementContract;
pub(super) fn load_measurement_contract() -> MeasurementContract {
    let contract: MeasurementContract = toml::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../config/gates/measurement-b4.toml"
    )))
    .unwrap();
    assert_eq!(contract.version, 1);
    assert_eq!(contract.profile, "CLASSICAL_DEV_LOCAL");
    assert_eq!(
        contract.scenario,
        "T-N09_RAM_RPC_WITH_PAUSED_WAL_CHECKPOINT_AND_ARCHIVE_MAINTENANCE"
    );
    assert_eq!(
        (
            contract.baseline_samples,
            contract.pressure_samples,
            contract.rpc_concurrency
        ),
        (128, 128, 2)
    );
    assert_eq!(contract.rpc_methods, ["eth_getBalance", "eth_call"]);
    assert_eq!(contract.rpc_transport, "production_http_loopback");
    assert_eq!(contract.simulation_arena_bytes, 8 * 1_048_576);
    assert_eq!(
        (
            contract.rpc_operation_timeout_ms,
            contract.small_fixture_rpc_p99_limit_ms
        ),
        (1_000, 100)
    );
    assert_eq!(
        contract.observed_process_peak_rss_limit_bytes,
        512 * 1_048_576
    );
    assert_eq!(contract.working_estimate_limit_bytes, 256 * 1_048_576);
    assert_eq!(contract.snapshot_staging_limit_bytes, 16 * 1_048_576);
    assert_eq!(contract.concurrent_storage_read_limit, 2);
    assert_eq!(contract.worker_cpu_basis_points, 2_500);
    assert_eq!(
        contract.worker_cpu_scope,
        "sole_writer_bounded_record_operation_cooperative_pacing"
    );
    assert_eq!(
        contract.worker_burst_policy,
        "one_physical_record_without_idle_credit"
    );
    assert_eq!(contract.maximum_physical_payload_bytes, 4_194_304);
    assert_eq!(contract.retained_parts, 4);
    assert_eq!(contract.encoded_queue_limit_bytes, 32 * 1_048_576);
    assert_eq!(contract.queue_age_admission_limit_ms, 2_000);
    assert_eq!(contract.logical_lag_limit_blocks, 2);
    assert!(
        contract.pressure_requires_actual_archive_compaction
            && contract.pressure_requires_actual_secondary_index_lookup
    );
    assert_eq!(contract.archive_compaction_operations, 128);
    assert_eq!(contract.archive_index_lookup_jobs, 128);
    assert_eq!(
        contract.archive_maintenance_scope,
        "separate_keep_all_and_canonical_history_namespaces"
    );
    assert_eq!(contract.maximum_concurrent_auxiliary_databases, 1);
    assert_eq!(
        contract.auxiliary_job_working_reservation_bytes,
        2 * 1_048_576
    );
    assert_eq!(contract.auxiliary_job_staging_reservation_bytes, 524_288);
    assert!(
        contract.pressure_requires_actual_wal_pause
            && contract.pressure_requires_actual_checkpoint_chunk_io
            && contract.pressure_requires_actual_checkpoint_proof_io
            && contract.compare_old_and_new_view_oracles
            && contract.verify_shutdown_and_reopen
    );
    assert!(
        !contract.os_memory_enforcement
            && !contract.os_cpu_enforcement
            && !contract.database_background_cpu_included_in_writer_pacing
            && !contract.throughput_or_mainnet_acceptance
    );
    contract
}
