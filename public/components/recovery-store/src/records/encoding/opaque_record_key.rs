// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::types::schema::RECORD_PREFIX;

pub(in crate::records) fn opaque_record_key(sequence: u64) -> Vec<u8> {
    [RECORD_PREFIX, sequence.to_be_bytes().as_slice()].concat()
}
