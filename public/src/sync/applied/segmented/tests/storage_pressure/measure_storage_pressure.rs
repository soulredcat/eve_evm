// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    archive_types::ArchiveJobCounts,
    initialize_archive_maintenance::initialize_archive_maintenance,
    run_archive_maintenance_jobs::run_archive_maintenance_jobs,
    run_checkpoint_io_jobs::run_checkpoint_io_jobs,
    sample_rpc_phase::sample_rpc_phase,
    types::{
        HttpRpcClients, MeasurementContract, PressureCoordination, PressureExecution,
        StorageJobChannels, StorageJobCoordination,
    },
};
use crate::sync::applied::{
    AppliedOwner, applied_owner_reader, tests::import_fixtures::ImportChain,
};
use eve_state::Address;
use serde_json::Value;
use std::sync::mpsc;

pub(super) fn measure_storage_pressure(
    owner: AppliedOwner,
    chain: &ImportChain,
    runtime: &tokio::runtime::Runtime,
    clients: HttpRpcClients,
    sender: Address,
    expected_balance: Value,
    contract: &MeasurementContract,
) -> PressureExecution {
    let reader = applied_owner_reader(&owner);
    let archive = match initialize_archive_maintenance(&reader, chain, contract) {
        Ok(archive) => archive,
        Err(error) => {
            return PressureExecution {
                metrics: Err(error),
                owner: Ok((owner, 0)),
                maintenance: Ok(ArchiveJobCounts::default()),
            };
        }
    };
    std::thread::scope(|scope| {
        let (run, requests) = mpsc::sync_channel(1);
        let (entered_sender, entered) = mpsc::sync_channel(1);
        let (done_sender, done) = mpsc::sync_channel(1);
        let (start_sender, start) = mpsc::sync_channel(1);
        let (maint_run, maint_requests) = mpsc::sync_channel(1);
        let (maint_entered_sender, maint_entered) = mpsc::sync_channel(1);
        let (maint_done_sender, maint_done) = mpsc::sync_channel(1);
        let (maint_start_sender, maint_start) = mpsc::sync_channel(1);
        let snapshot = scope.spawn(move || {
            run_checkpoint_io_jobs(
                owner,
                chain,
                StorageJobChannels {
                    run: requests,
                    entered: entered_sender,
                    done: done_sender,
                    start,
                },
            )
        });
        let maintenance = scope.spawn(move || {
            run_archive_maintenance_jobs(
                archive,
                reader,
                chain,
                contract,
                StorageJobChannels {
                    run: maint_requests,
                    entered: maint_entered_sender,
                    done: maint_done_sender,
                    start: maint_start,
                },
            )
        });
        let metrics = runtime.block_on(sample_rpc_phase(
            clients,
            sender,
            expected_balance,
            contract,
            Some(PressureCoordination {
                snapshot: StorageJobCoordination {
                    run: &run,
                    entered: &entered,
                    done: &done,
                    start: &start_sender,
                },
                maintenance: StorageJobCoordination {
                    run: &maint_run,
                    entered: &maint_entered,
                    done: &maint_done,
                    start: &maint_start_sender,
                },
            }),
        ));
        // Closing both request and release channels unblocks actors after any failed round.
        drop(run);
        drop(maint_run);
        drop(start_sender);
        drop(maint_start_sender);
        PressureExecution {
            metrics,
            owner: snapshot.join(),
            maintenance: maintenance.join(),
        }
    })
}
