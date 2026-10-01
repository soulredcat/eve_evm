// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::SigningError;
use crate::wire::tendermint::types::{BlockId, CanonicalBlockId, CanonicalPartSetHeader};

pub(crate) fn canonical_block_id(
    value: Option<&BlockId>,
) -> Result<Option<CanonicalBlockId>, SigningError> {
    let Some(block) = value else {
        return Ok(None);
    };
    let parts = block.part_set_header.clone().unwrap_or_default();
    if block.hash.is_empty() && parts.total == 0 && parts.hash.is_empty() {
        return Ok(None);
    }
    if block.hash.len() != 32 || parts.total == 0 || parts.hash.len() != 32 {
        return Err(SigningError::InvalidBlockId);
    }
    Ok(Some(CanonicalBlockId {
        hash: block.hash.clone(),
        part_set_header: Some(CanonicalPartSetHeader {
            total: parts.total,
            hash: parts.hash,
        }),
    }))
}
