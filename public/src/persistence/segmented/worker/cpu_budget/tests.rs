// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    begin_worker_cpu_record, create_worker_cpu_budget,
    observe_worker_cpu_budget::observe_worker_cpu_budget, pace_worker_cpu_record,
    required_worker_cpu_pacing_ns::required_worker_cpu_pacing_ns,
};
use crate::persistence::segmented::SegmentedError;

#[test]
fn checked_pacing_counts_io_elapsed_and_never_banks_idle_credit() {
    assert_eq!(required_worker_cpu_pacing_ns(1, 0, 2_500), Ok(4));
    assert_eq!(required_worker_cpu_pacing_ns(10, 40, 2_500), Ok(0));
    assert_eq!(required_worker_cpu_pacing_ns(10, 1_000_000, 2_500), Ok(0));
    assert_eq!(required_worker_cpu_pacing_ns(10, 0, 2_500), Ok(40));
    assert_eq!(required_worker_cpu_pacing_ns(1, 0, 3_333), Ok(4));
    assert!(matches!(
        required_worker_cpu_pacing_ns(1, 1, 0),
        Err(SegmentedError::InvalidConfiguration)
    ));
    assert!(matches!(
        required_worker_cpu_pacing_ns(1, 1, 10_001),
        Err(SegmentedError::InvalidConfiguration)
    ));
    assert!(matches!(
        required_worker_cpu_pacing_ns(u64::MAX, 0, 1),
        Err(SegmentedError::Overflow)
    ));
}
#[test]
#[cfg(target_os = "linux")]
fn real_thread_cpu_clock_and_actual_pacing_record_a_coherent_bounded_operation() {
    use sha2::{Digest, Sha256};
    let budget = create_worker_cpu_budget(2_500).unwrap();
    let window = begin_worker_cpu_record().unwrap();
    let bytes = vec![0x35; 65_536];
    for _ in 0..16 {
        std::hint::black_box(Sha256::digest(&bytes));
    }
    pace_worker_cpu_record(&budget, window).unwrap();
    let observation = observe_worker_cpu_budget(&budget).unwrap();
    assert_eq!(observation.basis_points, 2_500);
    assert_eq!(observation.completed_records, 1);
    assert!(observation.measured_cpu_ns > 0);
    assert!(
        u128::from(observation.measured_cpu_ns) * 10_000
            <= u128::from(observation.measured_wall_ns) * 2_500
    );
    assert_eq!(
        observation.maximum_record_burst_cpu_ns,
        observation.measured_cpu_ns
    );
}
#[test]
#[cfg(not(target_os = "linux"))]
fn unsupported_thread_cpu_platform_fails_closed() {
    assert!(matches!(
        create_worker_cpu_budget(2_500),
        Err(SegmentedError::InvalidConfiguration)
    ));
}
