// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{OpaqueRecordBudget, validate_opaque_record_budget};
use anyhow::{Context, Result};

/// Bounded canonical namespace read/decode and fixed control admission. The
/// caller holds an actual lease; RocksDB/OS resources retain their separate caps.
pub fn required_opaque_compaction_reservation(budget: &OpaqueRecordBudget) -> Result<usize> {
    validate_opaque_record_budget(budget)?;
    budget
        .maximum_read_bytes
        .checked_mul(2)
        .and_then(|bytes| bytes.checked_add(4_096))
        .context("opaque compaction read reservation overflow")
}
