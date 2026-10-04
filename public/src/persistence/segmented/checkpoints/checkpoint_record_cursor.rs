// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::SealedCheckpointRecord;
use eve_storage::records::OpaqueRecordCursor;
pub fn checkpoint_record_cursor(record: &SealedCheckpointRecord) -> OpaqueRecordCursor {
    record.0.cursor
}
