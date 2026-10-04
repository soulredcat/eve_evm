// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::persistence::segmented::SegmentedError;

/// Reviewed safe pinned rustix API. Unsupported platforms and unavailable clocks refuse.
pub(in crate::persistence::segmented) fn read_worker_thread_cpu_ns() -> Result<u64, SegmentedError>
{
    #[cfg(target_os = "linux")]
    {
        let time = rustix::time::clock_gettime_dynamic(rustix::time::DynamicClockId::Known(
            rustix::time::ClockId::ThreadCPUTime,
        ))
        .map_err(|_| SegmentedError::StorageFailed)?;
        let seconds = u64::try_from(time.tv_sec).map_err(|_| SegmentedError::Overflow)?;
        let nanos = u64::try_from(time.tv_nsec).map_err(|_| SegmentedError::Overflow)?;
        if nanos >= 1_000_000_000 {
            return Err(SegmentedError::StorageFailed);
        }
        seconds
            .checked_mul(1_000_000_000)
            .and_then(|value| value.checked_add(nanos))
            .ok_or(SegmentedError::Overflow)
    }
    #[cfg(not(target_os = "linux"))]
    {
        Err(SegmentedError::InvalidConfiguration)
    }
}
