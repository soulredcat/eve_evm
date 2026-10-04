// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use serde::Deserialize;
use std::sync::mpsc::SyncSender;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct MeasurementContract {
    pub version: u32,
    pub profile: String,
    pub scenario: String,
    pub baseline_samples: usize,
    pub pressure_samples: usize,
    pub rpc_concurrency: usize,
    pub rpc_methods: Vec<String>,
    pub rpc_transport: String,
    pub simulation_arena_bytes: usize,
    pub rpc_operation_timeout_ms: u64,
    pub small_fixture_rpc_p99_limit_ms: u64,
    pub observed_process_peak_rss_limit_bytes: u64,
    pub working_estimate_limit_bytes: u64,
    pub snapshot_staging_limit_bytes: u64,
    pub concurrent_storage_read_limit: u64,
    pub worker_cpu_basis_points: u64,
    pub worker_cpu_scope: String,
    pub worker_burst_policy: String,
    pub maximum_physical_payload_bytes: u64,
    pub retained_parts: usize,
    pub encoded_queue_limit_bytes: u64,
    pub queue_age_admission_limit_ms: u64,
    pub logical_lag_limit_blocks: usize,
    pub pressure_requires_actual_wal_pause: bool,
    pub pressure_requires_actual_checkpoint_chunk_io: bool,
    pub pressure_requires_actual_checkpoint_proof_io: bool,
    pub pressure_requires_actual_archive_compaction: bool,
    pub pressure_requires_actual_secondary_index_lookup: bool,
    pub archive_compaction_operations: usize,
    pub archive_index_lookup_jobs: usize,
    pub archive_maintenance_scope: String,
    pub primary_database_block_cache_bytes: usize,
    pub primary_database_write_buffer_bytes: usize,
    pub primary_database_write_buffer_count: i32,
    pub primary_database_background_jobs: i32,
    pub primary_database_open_files: i32,
    pub auxiliary_database_block_cache_bytes: usize,
    pub auxiliary_database_write_buffer_bytes: usize,
    pub auxiliary_database_write_buffer_count: i32,
    pub auxiliary_database_background_jobs: i32,
    pub auxiliary_database_open_files: i32,
    pub maximum_concurrent_auxiliary_databases: usize,
    pub auxiliary_job_working_reservation_bytes: usize,
    pub auxiliary_job_staging_reservation_bytes: usize,
    pub compare_old_and_new_view_oracles: bool,
    pub verify_shutdown_and_reopen: bool,
    pub os_memory_enforcement: bool,
    pub os_cpu_enforcement: bool,
    pub database_background_cpu_included_in_writer_pacing: bool,
    pub throughput_or_mainnet_acceptance: bool,
}
pub(super) struct PhaseMetrics {
    pub balance_p99_ns: u128,
    pub call_p99_ns: u128,
    pub sampled_peak_rss_bytes: u64,
    pub process_lifetime_hwm_bytes: u64,
}
pub(super) struct PressureFixture {
    pub directory: tempfile::TempDir,
    pub path: std::path::PathBuf,
    pub chain: crate::sync::applied::tests::import_fixtures::ImportChain,
    pub owner: crate::sync::applied::AppliedOwner,
    pub reader: crate::sync::applied::AppliedReader,
    pub genesis: std::sync::Arc<crate::sync::applied::AppliedPublication>,
    pub context: std::sync::Arc<crate::rpc::RpcContext>,
    pub clients: HttpRpcClients,
    pub servers: MeasurementServers,
    pub sender: eve_state::Address,
}
#[derive(Clone)]
pub(super) struct HttpRpcClients {
    pub balance: std::sync::Arc<jsonrpsee::http_client::HttpClient>,
    pub call: std::sync::Arc<jsonrpsee::http_client::HttpClient>,
}
pub(super) struct MeasurementServers {
    pub handles: Option<(
        jsonrpsee::server::ServerHandle,
        jsonrpsee::server::ServerHandle,
    )>,
}
impl Drop for MeasurementServers {
    fn drop(&mut self) {
        if let Some((http, ws)) = self.handles.take() {
            let _ = http.stop();
            let _ = ws.stop();
        }
    }
}
pub(super) struct ResumePausedWriter(pub Option<SyncSender<()>>);
#[derive(Clone, Copy)]
pub(super) struct StorageJobCoordination<'a> {
    pub run: &'a SyncSender<()>,
    pub entered: &'a std::sync::mpsc::Receiver<()>,
    pub done: &'a std::sync::mpsc::Receiver<Result<(), String>>,
    pub start: &'a SyncSender<()>,
}
pub(super) struct StorageJobChannels {
    pub run: std::sync::mpsc::Receiver<()>,
    pub entered: SyncSender<()>,
    pub done: SyncSender<Result<(), String>>,
    pub start: std::sync::mpsc::Receiver<()>,
}
#[derive(Clone, Copy)]
pub(super) struct PressureCoordination<'a> {
    pub snapshot: StorageJobCoordination<'a>,
    pub maintenance: StorageJobCoordination<'a>,
}
pub(super) struct PressureExecution {
    pub metrics: Result<PhaseMetrics, String>,
    pub owner: std::thread::Result<(crate::sync::applied::AppliedOwner, usize)>,
    pub maintenance: std::thread::Result<super::archive_types::ArchiveJobCounts>,
}
impl Drop for ResumePausedWriter {
    fn drop(&mut self) {
        if let Some(sender) = self.0.take() {
            let _ = sender.try_send(());
        }
    }
}
