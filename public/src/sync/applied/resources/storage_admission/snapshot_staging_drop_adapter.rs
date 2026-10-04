// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    AppliedSnapshotStagingReservation, release_snapshot_staging::release_snapshot_staging,
};
impl std::ops::Drop for AppliedSnapshotStagingReservation {
    fn drop(&mut self) {
        release_snapshot_staging(&self.pool, self.bytes);
    }
}
