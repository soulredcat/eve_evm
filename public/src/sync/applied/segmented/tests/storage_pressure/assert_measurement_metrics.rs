// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{MeasurementContract, PhaseMetrics};
pub(super) fn assert_measurement_metrics(phase: &PhaseMetrics, contract: &MeasurementContract) {
    let p99_ceiling = u128::from(contract.small_fixture_rpc_p99_limit_ms) * 1_000_000;
    assert!(
        phase.balance_p99_ns <= p99_ceiling,
        "predeclared balance p99 objective"
    );
    assert!(
        phase.call_p99_ns <= p99_ceiling,
        "predeclared eth_call p99 objective"
    );
    assert!(
        phase.sampled_peak_rss_bytes <= contract.observed_process_peak_rss_limit_bytes,
        "sampled scenario RSS objective; no OS hard memory quota is claimed"
    );
}
