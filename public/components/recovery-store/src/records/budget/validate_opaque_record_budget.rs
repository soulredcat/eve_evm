// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{OpaqueRecordBudget, types::schema::RECORD_HEADER_BYTES};
use anyhow::{Result, ensure};

pub fn validate_opaque_record_budget(budget: &OpaqueRecordBudget) -> Result<()> {
    ensure!(
        budget.maximum_record_bytes >= RECORD_HEADER_BYTES,
        "opaque record budget omits metadata"
    );
    ensure!(
        budget.maximum_batch_bytes >= budget.maximum_record_bytes,
        "opaque batch below record limit"
    );
    ensure!(
        budget.maximum_read_bytes >= budget.maximum_record_bytes,
        "opaque read below record limit"
    );
    ensure!(
        budget.maximum_batch_records > 0 && budget.maximum_retained_records > 0,
        "empty opaque count budget"
    );
    ensure!(
        budget
            .maximum_retained_records
            .checked_mul(u64::try_from(budget.maximum_record_bytes)?)
            .is_some(),
        "opaque retained byte limit overflows"
    );
    ensure!(
        budget.block_cache_bytes > 0 && budget.write_buffer_bytes > 0,
        "empty opaque DB memory budget"
    );
    ensure!(
        budget.write_buffer_count > 0
            && budget.maximum_background_jobs > 0
            && budget.maximum_open_files > 0,
        "invalid opaque DB worker/file budget"
    );
    Ok(())
}
