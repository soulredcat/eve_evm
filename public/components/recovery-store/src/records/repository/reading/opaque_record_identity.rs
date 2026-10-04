// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{OpaqueRecordIdentity, OpaqueRecordRepository};

/// Actual opened namespace identity; it grants no finality or external authority.
pub fn opaque_record_identity(repository: &OpaqueRecordRepository) -> OpaqueRecordIdentity {
    repository.identity
}
