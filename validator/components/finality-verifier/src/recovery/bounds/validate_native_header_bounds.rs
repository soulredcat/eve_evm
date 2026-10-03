// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::validate_native_block_id_bounds::validate_native_block_id_bounds;
use crate::recovery::RecoveryError;
use eve_consensus_comet::wire::tendermint::types::Header;

pub(crate) fn validate_native_header_bounds(header: &Header) -> Result<(), RecoveryError> {
    if header.chain_id.len() > 50
        || header.proposer_address.len() > 20
        || [
            &header.last_commit_hash,
            &header.data_hash,
            &header.validators_hash,
            &header.next_validators_hash,
            &header.consensus_hash,
            &header.app_hash,
            &header.last_results_hash,
            &header.evidence_hash,
        ]
        .iter()
        .any(|hash| hash.len() > 32)
    {
        return Err(RecoveryError::BudgetExceeded);
    }
    if let Some(id) = &header.last_block_id {
        validate_native_block_id_bounds(id)?;
    }
    Ok(())
}
