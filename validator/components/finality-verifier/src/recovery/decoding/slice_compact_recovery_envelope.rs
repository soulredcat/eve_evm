// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::take_length_prefixed::take_length_prefixed;
use crate::recovery::{
    RecoveryError,
    bounds::types::{
        MAXIMUM_NATIVE_FRAME_BYTES, MAXIMUM_RECOVERY_BYTES, MAXIMUM_VERSION_BYTES, RECOVERY_DOMAIN,
        RecoverySlices,
    },
};

pub(crate) fn slice_compact_recovery_envelope(
    bytes: &[u8],
) -> Result<RecoverySlices<'_>, RecoveryError> {
    if bytes.len() > MAXIMUM_RECOVERY_BYTES {
        return Err(RecoveryError::BudgetExceeded);
    }
    let mut remaining = bytes
        .strip_prefix(RECOVERY_DOMAIN)
        .ok_or(RecoveryError::MalformedEncoding)?;
    let fields = RecoverySlices {
        parent: take_length_prefixed(&mut remaining, MAXIMUM_VERSION_BYTES)?,
        expected: take_length_prefixed(&mut remaining, MAXIMUM_VERSION_BYTES)?,
        execution: take_length_prefixed(&mut remaining, MAXIMUM_RECOVERY_BYTES)?,
        finalized: take_length_prefixed(&mut remaining, MAXIMUM_NATIVE_FRAME_BYTES)?,
        lookahead: take_length_prefixed(&mut remaining, MAXIMUM_RECOVERY_BYTES)?,
    };
    if !remaining.is_empty() {
        return Err(RecoveryError::NonCanonicalEncoding);
    }
    Ok(fields)
}
