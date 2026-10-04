// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{super::types::MetadataLease, release_metadata::release_metadata};
impl std::ops::Drop for MetadataLease {
    fn drop(&mut self) {
        release_metadata(self);
    }
}
