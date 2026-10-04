// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ImportError;
use crate::{
    VerifiedDevelopmentHeader,
    recovery::{
        replay::derive_authenticated_execution_input::derive_authenticated_execution_input,
        types::capability::FixedGenesisPolicy,
    },
};
use eve_protocol_config::headers::derive_next_base_fee;
use eve_state::{Header, StateVersion};

/// One canonical consensus-derived execution environment for import/checkpoint validation.
pub(in crate::recovery::import) fn validate_authenticated_execution_context(
    parent_version: &StateVersion,
    parent_header: &Header,
    header: &Header,
    certified: &VerifiedDevelopmentHeader,
    policy: &FixedGenesisPolicy,
) -> Result<(), ImportError> {
    let agreed =
        derive_authenticated_execution_input(certified, policy).map_err(ImportError::Recovery)?;
    let base_fee = derive_next_base_fee(parent_header).map_err(ImportError::Header)?;
    if header.parent_hash != parent_version.execution_hash.0
        || header.timestamp != agreed.timestamp
        || header.beneficiary != agreed.proposer
        || header.mix_hash != agreed.previous_consensus_hash
        || header.gas_limit != parent_header.gas_limit
        || header.base_fee_per_gas != Some(base_fee)
    {
        return Err(ImportError::InvalidExecutionContext);
    }
    Ok(())
}
