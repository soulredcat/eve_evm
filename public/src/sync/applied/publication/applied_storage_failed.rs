// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedPublication;

pub fn applied_storage_failed(publication: &AppliedPublication) -> bool {
    publication.storage_failed
}
