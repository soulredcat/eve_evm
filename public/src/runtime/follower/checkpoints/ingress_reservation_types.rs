// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::{AppliedSnapshotStagingReservation, AppliedWorkingReservation};
/// Every client callback retains both dimensions with its owned response/witness.
pub(super) struct CheckpointIngressReservation {
    pub(super) _working: AppliedWorkingReservation,
    pub(super) _staging: AppliedSnapshotStagingReservation,
}
