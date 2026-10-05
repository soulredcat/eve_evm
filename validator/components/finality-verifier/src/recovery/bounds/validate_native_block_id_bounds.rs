// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::recovery::RecoveryError;
use eve_consensus_comet::wire::tendermint::types::BlockId;

pub(crate) fn validate_native_block_id_bounds(id: &BlockId) -> Result<(), RecoveryError> {
    if id.hash.len() > 32
        || id
            .part_set_header
            .as_ref()
            .is_some_and(|parts| parts.hash.len() > 32)
    {
        return Err(RecoveryError::BudgetExceeded);
    }
    Ok(())
}
