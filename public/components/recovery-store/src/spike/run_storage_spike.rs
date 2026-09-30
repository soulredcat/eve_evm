use std::{path::Path, time::Instant};

use anyhow::{Context, Result, ensure};

use crate::recovery::checkpoint::create_record_checkpoint;
use crate::recovery::opening::open_record_store;
use crate::recovery::records::{
    commit_record_batch, read_record, read_record_cursor, record_reader,
};
use crate::recovery::snapshot::{capture_record_snapshot, read_snapshot_record};
use crate::recovery::types::{StorageIdentity, StorageNetworkId};
use crate::spike::fixtures::build_spike_records;
use crate::spike::measure_directory_bytes::measure_directory_bytes;
use crate::spike::types::{StorageSpikeConfig, StorageSpikeReport};

pub fn run_storage_spike(path: &Path, config: StorageSpikeConfig) -> Result<StorageSpikeReport> {
    ensure!(
        path.is_absolute() && !path.exists(),
        "spike requires a new absolute task-owned directory"
    );
    ensure!(
        config.batch_count > 0 && config.records_per_batch > 0 && config.payload_bytes > 0,
        "empty spike workload"
    );
    ensure!(
        config.records_per_batch <= config.budget.max_batch_records,
        "spike batch-count budget exceeded"
    );
    let logical = config
        .batch_count
        .checked_mul(config.records_per_batch)
        .and_then(|count| count.checked_mul(config.payload_bytes))
        .context("spike workload overflow")?;
    ensure!(
        logical <= config.max_total_payload_bytes,
        "spike total-payload budget exceeded"
    );
    let identity = StorageIdentity {
        network: StorageNetworkId([0x42; 32]),
        genesis_hash: [0x43; 32],
        base_height: 0,
        base_block_hash: [0x44; 32],
    };
    std::fs::create_dir_all(path)?;
    let database_path = path.join("database");
    let checkpoint_path = path.join("checkpoint");
    let mut store = open_record_store(&database_path, identity, config.budget)?;
    ensure!(
        open_record_store(&database_path, identity, config.budget).is_err(),
        "namespace permitted second writer"
    );
    let reader = record_reader(&store);
    let before = capture_record_snapshot(&reader)?;
    let mut cursor = read_record_cursor(&store)?;
    let mut latencies = Vec::with_capacity(config.batch_count);
    for _ in 0..config.batch_count {
        let records = build_spike_records(cursor, config.records_per_batch, config.payload_bytes)?;
        let started = Instant::now();
        cursor = commit_record_batch(&mut store, &records)?;
        latencies.push(u64::try_from(started.elapsed().as_micros())?);
    }
    ensure!(
        before.cursor.height == 0 && read_snapshot_record(&before, 1)?.is_none(),
        "snapshot changed after commit"
    );
    drop(before);
    drop(reader);
    let started = Instant::now();
    let checkpoint_cursor = create_record_checkpoint(&mut store, &checkpoint_path)?;
    let checkpoint_latency_us = u64::try_from(started.elapsed().as_micros())?;
    ensure!(checkpoint_cursor == cursor, "checkpoint cursor differs");
    drop(store);
    let reopened = open_record_store(&database_path, identity, config.budget)?;
    ensure!(
        read_record_cursor(&reopened)? == cursor,
        "reopen lost synced cursor"
    );
    ensure!(
        read_record(&reopened, cursor.height)?.is_some(),
        "reopen lost synced block payload"
    );
    drop(reopened);
    let checkpoint = open_record_store(&checkpoint_path, identity, config.budget)?;
    ensure!(
        read_record_cursor(&checkpoint)? == cursor,
        "checkpoint lost synced cursor"
    );
    ensure!(
        read_record(&checkpoint, cursor.height)?.is_some(),
        "checkpoint lost block payload"
    );
    drop(checkpoint);
    Ok(StorageSpikeReport {
        profile: "B0-local-storage-interface-no-finality",
        batch_count: config.batch_count,
        records_per_batch: config.records_per_batch,
        payload_bytes: config.payload_bytes,
        budget: config.budget,
        logical_payload_bytes: u64::try_from(logical)?,
        sync_batch_latency_us: latencies,
        checkpoint_latency_us,
        database_bytes: measure_directory_bytes(&database_path)?,
        checkpoint_bytes: measure_directory_bytes(&checkpoint_path)?,
        durability_mode: "atomic WriteBatch; WAL enabled; sync=true",
        finality_verified: false,
        power_loss_tested: false,
    })
}
