// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{MAXIMUM_RECORD_PAYLOAD_BYTES, encode_record};
use crate::consensus::signing::types::SignedRecord;
use anyhow::{Result, ensure};

pub(in crate::consensus::signing) fn decode_record(bytes: &[u8]) -> Result<SignedRecord> {
    ensure!(
        bytes.len() <= MAXIMUM_RECORD_PAYLOAD_BYTES,
        "signer record decode budget exceeded"
    );
    let record = serde_json::from_slice(bytes)?;
    ensure!(
        encode_record(&record)? == bytes,
        "noncanonical signer record payload"
    );
    Ok(record)
}
