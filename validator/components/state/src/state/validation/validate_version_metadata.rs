// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_protocol_config::records::{ApplicationCommitmentInput, hash_application_commitment};

use super::validate_state_identity;
use crate::{StateError, StateVersion};

/// Local commitment consistency only; no complete content or finality is inferred.
pub(crate) fn validate_version_metadata(version: &StateVersion) -> Result<(), StateError> {
    validate_state_identity(&version.identity)?;
    let application = if version.height == 0 {
        None
    } else {
        Some(
            hash_application_commitment(ApplicationCommitmentInput {
                genesis: version.identity.genesis,
                protocol_version: version.identity.protocol_version,
                execution_height: version.height,
                evm_root: version.evm_root,
                system_root: version.system_root,
                execution_hash: version.execution_hash,
            })
            .map_err(StateError::SystemRecord)?,
        )
    };
    if application != version.application {
        return Err(StateError::VersionMismatch);
    }
    Ok(())
}
