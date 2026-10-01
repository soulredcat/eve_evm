// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::OpaqueRecordBudget;

/// Finite development limits; exhaustion stops appends and never prunes safety history.
pub fn development_opaque_record_budget() -> OpaqueRecordBudget {
    OpaqueRecordBudget {
        maximum_record_bytes: 4 * 1_048_576 + 4096,
        maximum_batch_bytes: 16 * 1_048_576,
        maximum_batch_records: 64,
        maximum_read_bytes: 4 * 1_048_576 + 4096,
        maximum_retained_records: 1_000_000,
        block_cache_bytes: 4 * 1_048_576,
        write_buffer_bytes: 4 * 1_048_576,
        write_buffer_count: 2,
        maximum_background_jobs: 1,
        maximum_open_files: 64,
    }
}
