// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::AppliedError;
use eve_state::{StateVersion, encode_state_version};
use sha2::{Digest, Sha256};

/// Local canonical representation identity, including auxiliary digest; no certificate authority.
pub(in crate::sync::applied) fn bind_local_state_version(
    version: &StateVersion,
) -> Result<[u8; 32], AppliedError> {
    let bytes = encode_state_version(version).map_err(|_| AppliedError::InvalidDurablePrefix)?;
    let mut hash = Sha256::new();
    hash.update(b"EVE_PUBLIC_SEGMENTED_STATE_BINDING_V1");
    hash.update(bytes);
    Ok(hash.finalize().into())
}
