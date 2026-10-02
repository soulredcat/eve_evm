// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{OpaqueRecordBudget, OpaqueRecordRepository};

/// Read actual immutable repository limits without exposing the database or write authority.
pub fn opaque_record_budget(repository: &OpaqueRecordRepository) -> OpaqueRecordBudget {
    repository.budget
}
