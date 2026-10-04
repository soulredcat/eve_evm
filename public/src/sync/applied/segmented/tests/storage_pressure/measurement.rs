// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    assert_measurement_metrics::assert_measurement_metrics,
    create_pressure_fixture::create_pressure_fixture,
    load_measurement_contract::load_measurement_contract,
    measure_storage_pressure::measure_storage_pressure,
    pressure_configuration::pressure_configuration,
    print_measurement_metrics::print_measurement_metrics,
    sample_rpc_phase::sample_rpc_phase,
    stop_measurement_http::stop_measurement_http,
    types::{PressureFixture, ResumePausedWriter},
};
use crate::{
    persistence::segmented::{
        install_segmented_record_pause, observe_segmented_parts, observe_segmented_worker_cpu,
    },
    sync::applied::{types::AppliedBackend, *},
};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

#[test]
fn t_n09_actual_checkpoint_io_and_paused_wal_keep_ram_rpc_bounded_under_frozen_local_contract() {
    let contract = load_measurement_contract();
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    let _runtime_context = runtime.enter();
    let PressureFixture {
        directory: _directory,
        path,
        chain,
        mut owner,
        reader,
        genesis,
        context,
        clients,
        servers,
        sender,
    } = create_pressure_fixture(&runtime, &contract);
    let baseline = runtime
        .block_on(sample_rpc_phase(
            clients.clone(),
            sender,
            crate::rpc::encoding::quantity(chain.commits[0].state.accounts[&sender].balance),
            &contract,
            None,
        ))
        .unwrap();
    let AppliedBackend::Segmented { worker, pool, .. } = &owner.backend else {
        panic!("segmented owner");
    };
    let queue_pool = Arc::clone(pool);
    let (entered, resume) = install_segmented_record_pause(worker.as_ref().unwrap(), 0, false);
    let resume = ResumePausedWriter(Some(resume));
    try_apply_recovery_bytes(&mut owner, &chain.records[0]).unwrap();
    entered.recv_timeout(Duration::from_secs(10)).unwrap();
    let applied = capture_applied_state(&reader).unwrap();
    runtime
        .block_on(context.pool.applied_committed(Arc::clone(&applied)))
        .unwrap();
    let execution = measure_storage_pressure(
        owner,
        &chain,
        &runtime,
        clients,
        sender,
        crate::rpc::encoding::quantity(chain.commits[1].state.accounts[&sender].balance),
        &contract,
    );
    let (mut owner, completed_jobs) = match execution.owner {
        Ok(returned) => returned,
        Err(panic) => {
            drop(resume);
            runtime.block_on(stop_measurement_http(servers));
            std::panic::resume_unwind(panic)
        }
    };
    let queued = observe_segmented_parts(&queue_pool).unwrap();
    let queue_age = queued.oldest_age.as_millis();
    let age_refusal = if queue_age > u128::from(contract.queue_age_admission_limit_ms) {
        Some(try_apply_recovery_bytes(&mut owner, &chain.records[1]))
    } else {
        None
    };
    let after_refusal = observe_segmented_parts(&queue_pool).unwrap();
    let after_pressure = capture_applied_state(&reader).unwrap();
    // Release the real writer on every later assertion/error path; no paused test thread is abandoned.
    drop(resume);
    let deadline = Instant::now() + Duration::from_secs(10);
    while poll_applied_durability(&mut owner)
        .unwrap()
        .durable_recovery
        .0
        != 1
    {
        assert!(Instant::now() < deadline);
        std::thread::yield_now();
    }
    let cpu = match &owner.backend {
        AppliedBackend::Segmented { worker, .. } => {
            observe_segmented_worker_cpu(worker.as_ref().unwrap()).unwrap()
        }
        _ => unreachable!(),
    };
    let storage = observe_applied_storage(&reader).unwrap();
    let working = observe_estimated_working(&reader).unwrap();
    let shutdown = finish_applied_state_service(owner);
    runtime.block_on(stop_measurement_http(servers));
    assert!(shutdown.acknowledgement_error.is_none());
    assert_eq!(retained_tail_len(&shutdown.unacknowledged_tail), 0);
    assert_eq!(completed_jobs, contract.pressure_samples);
    let pressure = execution.metrics.unwrap();
    let maintenance = execution.maintenance.unwrap();
    assert_eq!(
        maintenance.compactions,
        contract.archive_compaction_operations
    );
    assert_eq!(
        maintenance.index_lookups,
        contract.archive_index_lookup_jobs
    );
    assert_measurement_metrics(&baseline, &contract);
    assert_measurement_metrics(&pressure, &contract);
    assert_eq!(applied_commit(&genesis), &chain.commits[0]);
    assert_eq!(applied_commit(&applied), &chain.commits[1]);
    assert!(Arc::ptr_eq(&applied, &after_pressure));
    assert_eq!(applied_markers(&applied).durable_recovery.0, 0);
    assert!(matches!(
        applied_readiness(&applied),
        eve_node_policy::PublicReadiness::NotReady(_)
    ));
    assert!(queued.retained_parts <= contract.retained_parts);
    assert!(queued.retained_encoded_bytes <= contract.encoded_queue_limit_bytes);
    assert_eq!(after_refusal.retained_parts, queued.retained_parts);
    assert_eq!(
        after_refusal.retained_encoded_bytes,
        queued.retained_encoded_bytes
    );
    assert_eq!(
        after_refusal.estimated_metadata_bytes,
        queued.estimated_metadata_bytes
    );
    if let Some(refused) = age_refusal {
        assert!(matches!(refused, Err(AppliedError::QueueLimit)));
    }
    assert_eq!(storage.active_reads, 0);
    assert_eq!(storage.reserved_staging_bytes, 0);
    assert!(storage.peak_reads <= contract.concurrent_storage_read_limit);
    assert!(storage.peak_staging_bytes <= contract.snapshot_staging_limit_bytes);
    assert!(working.peak_estimated_bytes <= contract.working_estimate_limit_bytes);
    assert!(cpu.completed_records > 0 && cpu.measured_cpu_ns > 0);
    assert!(
        u128::from(cpu.measured_cpu_ns) * 10_000
            <= u128::from(cpu.measured_wall_ns) * u128::from(contract.worker_cpu_basis_points)
    );
    let durable = shutdown.publication.as_ref().unwrap();
    assert_eq!(applied_commit(durable), &chain.commits[1]);
    assert_eq!(applied_markers(durable).durable_recovery.0, 1);
    drop(shutdown);
    drop(context);
    let (reopened, restored_reader) = open_segmented_applied_state_service(
        pressure_configuration(&path, &chain, &contract),
        &chain.genesis,
    )
    .unwrap();
    assert_eq!(
        applied_commit(&capture_applied_state(&restored_reader).unwrap()),
        &chain.commits[1]
    );
    drop(finish_applied_state_service(reopened));
    print_measurement_metrics(
        &baseline,
        &pressure,
        cpu,
        storage,
        working,
        queue_age,
        maintenance,
    );
}
