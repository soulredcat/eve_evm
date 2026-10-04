// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::types::ScratchLease;
use super::release_segmented_scratch::release_segmented_scratch;
impl std::ops::Drop for ScratchLease {
    fn drop(&mut self) {
        release_segmented_scratch(self);
    }
}
