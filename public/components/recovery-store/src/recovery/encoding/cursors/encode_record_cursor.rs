// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::encoding::identity::encode_store_identity::encode_store_identity;
use crate::recovery::types::DurableRecordCursor;

pub fn encode_record_cursor(cursor: &DurableRecordCursor) -> Vec<u8> {
    let mut bytes = encode_store_identity(&cursor.identity);
    bytes.extend_from_slice(&cursor.height.to_be_bytes());
    bytes.extend_from_slice(&cursor.block_hash);
    bytes
}
