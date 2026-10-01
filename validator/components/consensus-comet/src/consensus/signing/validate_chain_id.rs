// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{SigningError, types::MAX_CHAIN_ID_BYTES};

pub(crate) fn validate_chain_id(chain_id: &str) -> Result<(), SigningError> {
    if chain_id.is_empty()
        || chain_id.len() > MAX_CHAIN_ID_BYTES
        || chain_id.chars().any(char::is_control)
    {
        return Err(SigningError::InvalidChainId);
    }
    Ok(())
}
