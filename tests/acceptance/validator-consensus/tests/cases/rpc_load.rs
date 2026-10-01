// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::{
    Cluster, ClusterOptions,
    cluster::{node_height, wait_for_height},
    cluster_lease, collect_certified_history,
    process::count_owned_tcp_connections,
    rpc::rpc_json,
};
use anyhow::{Context, Result, ensure};
use eve_consensus_comet::consensus::certificates::validator_address;
use std::{
    collections::BTreeMap,
    io::Write,
    net::{Shutdown, TcpStream},
    time::{Duration, Instant},
};

#[test]
fn t_c10_native_rpc_connection_quota_pressure_preserves_progress_with_master_absent() -> Result<()>
{
    let _lease = cluster_lease();
    let mut cluster = Cluster::create("tc10", ClusterOptions::default())?;
    cluster.start()?;
    let before = node_height(&cluster, 1)?;
    let address = cluster.nodes[0].rpc;
    let pid = cluster.nodes[0]
        .engine_pid
        .context("actual native PID missing")?;
    let mut slow = Vec::new();
    let mut refused = 0;
    for _ in 0..96 {
        match TcpStream::connect_timeout(&address, Duration::from_millis(100)) {
            Ok(mut stream) => {
                stream.set_write_timeout(Some(Duration::from_millis(100)))?;
                stream.write_all(b"GET /status HTTP/1.1\r\nHost: localhost\r\n")?;
                slow.push(stream);
            }
            Err(_) => refused += 1,
        }
    }
    let saturated = Instant::now() + Duration::from_secs(2);
    while count_owned_tcp_connections(pid, address.port())? < 64 {
        ensure!(
            Instant::now() < saturated,
            "did not exercise actual configured 64-connection native RPC quota"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    let mut overflow = Vec::new();
    for _ in 0..8 {
        overflow.push(std::thread::spawn(move || {
            rpc_json(address, "/status").is_err()
        }));
    }
    let overlap = Instant::now() + Duration::from_secs(5);
    while node_height(&cluster, 1)? < before + 2 {
        ensure!(
            Instant::now() < overlap,
            "validator consensus stalled while native RPC quota was occupied"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
    ensure!(
        count_owned_tcp_connections(pid, address.port())? >= 64,
        "pressure ended before measured native progress"
    );
    let rejected = overflow
        .into_iter()
        .map(|worker| usize::from(worker.join().unwrap()))
        .sum::<usize>();
    ensure!(
        rejected > 0 && slow.len() + refused == 96,
        "bounded quota packet had no measured rejected/deadline-limited request"
    );
    std::fs::write(
        cluster.artifact.join("rpc-pressure.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "slow_connections_established": slow.len(),
            "slow_connections_refused": refused,
            "overflow_requests": 8,
            "overflow_rejected": rejected,
            "overlap_progress_blocks": node_height(&cluster, 1)? - before,
            "native_connection_quota": 64
        }))?,
    )?;
    for stream in slow {
        let _ = stream.shutdown(Shutdown::Both);
    }
    wait_for_height(&mut cluster, &[0, 1, 2, 3], before + 3)?;
    let history = collect_certified_history(&cluster, 1, before + 2, &BTreeMap::new())?;
    let address = validator_address(&cluster.nodes[0].public_key);
    ensure!(
        history
            .iter()
            .filter(|block| block.block.header.height > before)
            .any(|block| block
                .commit
                .signatures
                .iter()
                .any(|signature| signature.block_id_flag == 2
                    && signature.validator_address == address)),
        "RPC-pressured native validator did not contribute an authenticated overlapping commit vote"
    );
    cluster.stop()?;
    Ok(())
}
