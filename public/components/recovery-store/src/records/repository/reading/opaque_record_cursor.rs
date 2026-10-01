// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{OpaqueRecordCursor, OpaqueRecordRepository};
use anyhow::{Result, ensure};
use std::sync::atomic::Ordering;

/// Return the acknowledged RAM head only while the exclusive repository is unfenced.
pub fn opaque_record_cursor(store: &OpaqueRecordRepository) -> Result<OpaqueRecordCursor> {
    ensure!(
        !store.fenced.load(Ordering::Acquire),
        "opaque handle fenced; drop and reopen/reconcile"
    );
    Ok(store.head)
}
