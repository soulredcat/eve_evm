// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    initialize_state_snapshot::initialize_state_snapshot,
    release_snapshot_lease::release_snapshot_lease,
};
use crate::state::{StateReader, StateSnapshot};
use anyhow::{Result, anyhow, ensure};
use std::sync::atomic::Ordering;

pub fn capture_state_snapshot(reader: &StateReader) -> Result<StateSnapshot<'_>> {
    ensure!(
        !reader.fence.load(Ordering::Acquire),
        "state reader fenced; reopen/reconcile before fresh capture"
    );
    reader
        .snapshots
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |current| {
            if current < reader.budget.maximum_snapshots {
                Some(current + 1)
            } else {
                None
            }
        })
        .map_err(|_| anyhow!("concurrent state snapshot budget exceeded"))?;
    let result = initialize_state_snapshot(reader);
    if result.is_err() {
        release_snapshot_lease(&reader.snapshots);
    }
    result
}
