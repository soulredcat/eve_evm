// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    AuthenticatedImportInput, ImportError,
    validate_authenticated_execution_context::validate_authenticated_execution_context,
};
use crate::{VerifiedDevelopmentHeader, recovery::types::capability::FixedGenesisPolicy};
use eve_state::StateCommit;

/// Preserve full import's actual parent/input and delegate the canonical environment checks.
pub(super) fn validate_import_execution_context(
    parent: &StateCommit,
    input: &AuthenticatedImportInput,
    certified: &VerifiedDevelopmentHeader,
    policy: &FixedGenesisPolicy,
) -> Result<(), ImportError> {
    validate_authenticated_execution_context(
        &parent.target,
        &parent.block.header,
        &input.execution.header,
        certified,
        policy,
    )
}
