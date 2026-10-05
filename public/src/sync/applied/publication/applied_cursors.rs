// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::sync::applied::AppliedPublication;
use eve_storage::records::OpaqueRecordCursor;

/// Admission and verified durable-prefix cursors belong to this same captured publication.
pub fn applied_cursors(
    publication: &AppliedPublication,
) -> (OpaqueRecordCursor, OpaqueRecordCursor) {
    (publication.admitted_cursor, publication.durable_cursor)
}
