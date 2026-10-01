// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::Result;
use eve_storage::recovery::types::StorageBudget;
use eve_storage::spike::{run_storage_spike, types::StorageSpikeConfig};

fn spike_config() -> StorageSpikeConfig {
    StorageSpikeConfig {
        batch_count: 4,
        records_per_batch: 8,
        payload_bytes: 256,
        max_total_payload_bytes: 8192,
        budget: StorageBudget {
            max_record_bytes: 4096,
            max_batch_bytes: 65536,
            max_batch_records: 16,
            write_buffer_bytes: 4 * 1024 * 1024,
            write_buffer_count: 2,
            block_cache_bytes: 4 * 1024 * 1024,
            max_background_jobs: 2,
            max_open_files: 32,
        },
    }
}

#[test]
fn bounded_spike_measures_real_sync_checkpoint_and_reopen() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let config = spike_config();
    let report = run_storage_spike(&directory.path().join("spike"), config)?;
    assert_eq!(report.logical_payload_bytes, 8192);
    assert_eq!(report.sync_batch_latency_us.len(), 4);
    assert!(report.database_bytes > report.logical_payload_bytes);
    assert!(report.checkpoint_bytes > 0);
    assert!(!report.finality_verified && !report.power_loss_tested);
    println!("{}", serde_json::to_string(&report)?);
    Ok(())
}

#[test]
fn spike_refuses_preexisting_destination_or_over_budget_workload() -> Result<()> {
    let directory = tempfile::tempdir()?;
    let mut config = spike_config();
    config.max_total_payload_bytes = 8191;
    let destination = directory.path().join("over-budget");
    assert!(run_storage_spike(&destination, config).is_err());
    assert!(!destination.exists());
    config.max_total_payload_bytes = 8192;
    assert!(run_storage_spike(directory.path(), config).is_err());
    Ok(())
}
