// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::types::ScratchLease;
use std::sync::atomic::Ordering;
pub(super) fn release_segmented_scratch(lease: &ScratchLease) {
    lease.state.scratch.store(0, Ordering::Release);
}
