// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::{OpaqueRecordCursor, OpaqueRecordRepository, hashing::hash_opaque_record};

/// Physical-zero binding of the actual opened namespace, not state or finality.
pub fn opaque_record_bootstrap_cursor(repository: &OpaqueRecordRepository) -> OpaqueRecordCursor {
    OpaqueRecordCursor {
        sequence: 0,
        content_hash: hash_opaque_record(
            repository.identity,
            OpaqueRecordCursor::default(),
            0,
            &[],
        ),
    }
}
