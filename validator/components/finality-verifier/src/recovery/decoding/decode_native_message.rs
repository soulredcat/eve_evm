// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::RecoveryError;
use prost::Message;

/// Caller must first scan the pinned message topology and bounds without allocation.
pub(crate) fn decode_native_message<M: Message + Default>(
    input: &[u8],
) -> Result<M, RecoveryError> {
    let message = M::decode(input).map_err(|_| RecoveryError::MalformedEncoding)?;
    if message.encode_to_vec() != input {
        return Err(RecoveryError::NonCanonicalEncoding);
    }
    Ok(message)
}
