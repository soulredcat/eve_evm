// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::MAXIMUM_RECORD_PAYLOAD_BYTES;
use crate::consensus::signing::types::SignedRecord;
use anyhow::{Result, ensure};

pub(in crate::consensus::signing) fn encode_record(record: &SignedRecord) -> Result<Vec<u8>> {
    let bytes = serde_json::to_vec(record)?;
    ensure!(
        bytes.len() <= MAXIMUM_RECORD_PAYLOAD_BYTES,
        "signer record payload budget exceeded"
    );
    Ok(bytes)
}
