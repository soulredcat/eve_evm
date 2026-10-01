// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{RecordError, SystemRecord, encode_system_record};

/// After typed decoding, reject alternate/noncanonical bytes; not a provenance proof.
pub fn validate_canonical_record_bytes(
    record: &SystemRecord,
    bytes: &[u8],
) -> Result<(), RecordError> {
    if encode_system_record(record)? != bytes {
        return Err(RecordError::NonCanonical);
    }
    Ok(())
}
