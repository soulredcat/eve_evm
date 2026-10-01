// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{decode_version::decode_version, encode_state_version};
use crate::{StateError, StateVersion};

pub fn decode_state_version(bytes: &[u8]) -> Result<StateVersion, StateError> {
    if bytes.len() > 4_096 {
        return Err(StateError::BudgetExceeded);
    }
    let mut remaining = bytes;
    let version = decode_version(&mut remaining)?;
    if !remaining.is_empty() || encode_state_version(&version)?.as_ref() != bytes {
        return Err(StateError::NonCanonicalEncoding);
    }
    Ok(version)
}
