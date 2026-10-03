// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_protocol_config::headers::derive_next_base_fee;
use eve_state::StateCommit;

use super::{AuthenticatedImportInput, ImportError};
use crate::{
    VerifiedDevelopmentHeader,
    recovery::{
        replay::derive_authenticated_execution_input::derive_authenticated_execution_input,
        types::capability::FixedGenesisPolicy,
    },
};

/// Check the consensus-derived environment independently of EVM execution.
pub(super) fn validate_import_execution_context(
    parent: &StateCommit,
    input: &AuthenticatedImportInput,
    certified: &VerifiedDevelopmentHeader,
    policy: &FixedGenesisPolicy,
) -> Result<(), ImportError> {
    let agreed =
        derive_authenticated_execution_input(certified, policy).map_err(ImportError::Recovery)?;
    let base_fee = derive_next_base_fee(&parent.block.header).map_err(ImportError::Header)?;
    let header = &input.execution.header;
    if header.parent_hash != parent.target.execution_hash.0
        || header.timestamp != agreed.timestamp
        || header.beneficiary != agreed.proposer
        || header.mix_hash != agreed.previous_consensus_hash
        || header.gas_limit != parent.block.header.gas_limit
        || header.base_fee_per_gas != Some(base_fee)
    {
        return Err(ImportError::InvalidExecutionContext);
    }
    Ok(())
}
