// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::OpaqueRecordCursor;

pub(in crate::records) fn encode_opaque_cursor(cursor: OpaqueRecordCursor) -> [u8; 40] {
    let mut bytes = [0; 40];
    bytes[..8].copy_from_slice(&cursor.sequence.to_be_bytes());
    bytes[8..].copy_from_slice(&cursor.content_hash);
    bytes
}
