// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::MeasurementContract;
use crate::sync::applied::{
    SegmentedAppliedConfig, segmented::tests::fixtures::configuration,
    tests::import_fixtures::ImportChain,
};
use std::path::Path;

/// Reserve one auxiliary DB slice without increasing any production or aggregate profile limit.
pub(super) fn pressure_configuration(
    path: &Path,
    chain: &ImportChain,
    contract: &MeasurementContract,
) -> SegmentedAppliedConfig {
    let mut config = configuration(path, chain);
    let limits = config.application.public_budget;
    assert_eq!(
        contract.primary_database_block_cache_bytes + contract.auxiliary_database_block_cache_bytes,
        limits.block_cache_bytes as usize
    );
    assert_eq!(
        contract.primary_database_write_buffer_bytes
            + contract.auxiliary_database_write_buffer_bytes,
        limits.write_buffer_bytes as usize
    );
    assert_eq!(
        contract.primary_database_write_buffer_count
            + contract.auxiliary_database_write_buffer_count,
        i32::try_from(limits.write_buffer_count).unwrap()
    );
    assert_eq!(
        contract.primary_database_background_jobs + contract.auxiliary_database_background_jobs,
        i32::try_from(limits.background_jobs).unwrap()
    );
    assert_eq!(
        contract.primary_database_open_files + contract.auxiliary_database_open_files,
        i32::try_from(limits.open_files).unwrap()
    );
    let repository = &mut config.application.repository_budget;
    repository.block_cache_bytes = contract.primary_database_block_cache_bytes;
    repository.write_buffer_bytes = contract.primary_database_write_buffer_bytes;
    repository.write_buffer_count = contract.primary_database_write_buffer_count;
    repository.maximum_background_jobs = contract.primary_database_background_jobs;
    repository.maximum_open_files = contract.primary_database_open_files;
    config
}
