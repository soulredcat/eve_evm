// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    read_process_rss::read_process_rss,
    timed_rpc::timed_rpc,
    types::{HttpRpcClients, MeasurementContract, PhaseMetrics, PressureCoordination},
};
use eve_state::Address;
use serde_json::{Value, json};
use std::{sync::Arc, time::Duration};

/// Every round uses two real production RPC operations; pressure job barriers surround actual checkpoint IO.
pub(super) async fn sample_rpc_phase(
    clients: HttpRpcClients,
    sender: Address,
    expected_balance: Value,
    contract: &MeasurementContract,
    pressure: Option<PressureCoordination<'_>>,
) -> Result<PhaseMetrics, String> {
    let count = if pressure.is_some() {
        contract.pressure_samples
    } else {
        contract.baseline_samples
    };
    let mut balances = Vec::with_capacity(count);
    let mut calls = Vec::with_capacity(count);
    let (mut rss_peak, mut hwm) = read_process_rss();
    for _ in 0..count {
        if let Some(coordination) = pressure {
            for job in [coordination.snapshot, coordination.maintenance] {
                job.run
                    .send(())
                    .map_err(|_| "storage IO actor unavailable")?;
            }
            for job in [coordination.snapshot, coordination.maintenance] {
                job.entered
                    .recv_timeout(Duration::from_secs(1))
                    .map_err(|_| "storage IO readiness timeout")?;
            }
            // Both actors are ready before either performs the measured round's IO.
            for job in [coordination.snapshot, coordination.maintenance] {
                job.start
                    .send(())
                    .map_err(|_| "storage IO release unavailable")?;
            }
        }
        let (balance, call) = tokio::join!(
            timed_rpc(
                Arc::clone(&clients.balance),
                "eth_getBalance",
                vec![json!(sender), json!("latest")],
                contract.rpc_operation_timeout_ms
            ),
            timed_rpc(
                Arc::clone(&clients.call),
                "eth_call",
                vec![
                    json!({"from":sender,"to":Address::with_last_byte(0x42)}),
                    json!("latest")
                ],
                contract.rpc_operation_timeout_ms
            )
        );
        if let Some(coordination) = pressure {
            for job in [coordination.snapshot, coordination.maintenance] {
                job.done
                    .recv_timeout(Duration::from_secs(5))
                    .map_err(|_| "storage IO completion timeout")??;
            }
        }
        let (balance, balance_ns) = balance?;
        let (call, call_ns) = call?;
        if balance != expected_balance || call != json!("0x") {
            return Err("RAM RPC result differs from canonical execution oracle".into());
        }
        balances.push(balance_ns);
        calls.push(call_ns);
        let (rss, current_hwm) = read_process_rss();
        rss_peak = rss_peak.max(rss);
        hwm = hwm.max(current_hwm);
    }
    balances.sort_unstable();
    calls.sort_unstable();
    let position = count.checked_mul(99).unwrap().div_ceil(100) - 1;
    Ok(PhaseMetrics {
        balance_p99_ns: balances[position],
        call_p99_ns: calls[position],
        sampled_peak_rss_bytes: rss_peak,
        process_lifetime_hwm_bytes: hwm,
    })
}
