// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    archive_types::{ArchiveJobCounts, ArchiveMaintenance},
    compact_retained_archive::compact_retained_archive,
    lookup_indexed_archive::lookup_indexed_archive,
    types::{MeasurementContract, StorageJobChannels},
};
use crate::sync::applied::{
    AppliedReader, reserve_applied_snapshot_staging, reserve_applied_storage_read,
    reserve_applied_working, tests::import_fixtures::ImportChain,
};

/// The archive actor is independent of the paused writer and shares the same actual two-read controller.
pub(super) fn run_archive_maintenance_jobs(
    archive: ArchiveMaintenance,
    reader: AppliedReader,
    chain: &ImportChain,
    contract: &MeasurementContract,
    channels: StorageJobChannels,
) -> ArchiveJobCounts {
    let mut counts = ArchiveJobCounts::default();
    while channels.run.recv().is_ok() {
        let result = (|| -> Result<(), String> {
            let _read =
                reserve_applied_storage_read(&reader).map_err(|_| "maintenance read capacity")?;
            let _working =
                reserve_applied_working(&reader, contract.auxiliary_job_working_reservation_bytes)
                    .map_err(|_| "maintenance working capacity")?;
            let _raw = reserve_applied_snapshot_staging(
                &reader,
                contract.auxiliary_job_staging_reservation_bytes,
            )
            .map_err(|_| "maintenance staging capacity")?;
            channels
                .entered
                .send(())
                .map_err(|_| "maintenance IO barrier closed")?;
            channels
                .start
                .recv()
                .map_err(|_| "maintenance IO release closed")?;
            // Each helper closes its exclusive DB handle before the next namespace opens.
            compact_retained_archive(&archive, chain)?;
            counts.compactions += 1;
            lookup_indexed_archive(&archive, chain)?;
            counts.index_lookups += 1;
            Ok(())
        })();
        if channels.done.send(result).is_err() {
            break;
        }
    }
    counts
}
