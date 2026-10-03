// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{super::types::PartLease, release_part::release_part};
impl std::ops::Drop for PartLease {
    fn drop(&mut self) {
        release_part(self);
    }
}
