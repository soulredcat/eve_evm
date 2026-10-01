// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};

use crate::recovery::types::{RECORD_HEADER_BYTES, StorageBudget};

pub fn validate_storage_budget(budget: &StorageBudget) -> Result<()> {
    ensure!(
        budget.max_record_bytes > RECORD_HEADER_BYTES,
        "record budget must allow payload"
    );
    ensure!(
        budget.max_batch_bytes >= budget.max_record_bytes,
        "batch budget below record budget"
    );
    ensure!(budget.max_batch_records > 0, "empty batch-count budget");
    ensure!(
        budget.write_buffer_bytes > 0 && budget.block_cache_bytes > 0,
        "empty memory budget"
    );
    ensure!(
        budget.max_background_jobs > 0 && budget.max_open_files > 0,
        "invalid worker/file budget"
    );
    ensure!(budget.write_buffer_count > 0, "invalid write-buffer count");
    Ok(())
}
