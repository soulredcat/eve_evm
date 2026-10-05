// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{EstimatedWorkingLease, release_estimated_working::release_estimated_working};

impl std::ops::Drop for EstimatedWorkingLease {
    fn drop(&mut self) {
        release_estimated_working(self);
    }
}
