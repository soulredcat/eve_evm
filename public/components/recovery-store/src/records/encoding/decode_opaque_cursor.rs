// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::records::OpaqueRecordCursor;
use anyhow::{Result, ensure};

pub(in crate::records) fn decode_opaque_cursor(bytes: &[u8]) -> Result<OpaqueRecordCursor> {
    ensure!(bytes.len() == 40, "invalid opaque cursor length");
    Ok(OpaqueRecordCursor {
        sequence: u64::from_be_bytes(bytes[..8].try_into()?),
        content_hash: bytes[8..].try_into()?,
    })
}
