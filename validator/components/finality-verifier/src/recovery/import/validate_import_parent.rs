// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{AuthenticatedImportInput, ImportError, ImportedState};

/// Exact local auxiliary identity is intentional; no peer-supplied target digest
/// enters this contract and no imported state is converted into a replay capability.
pub(super) fn validate_import_parent(
    parent: &ImportedState,
    input: &AuthenticatedImportInput,
) -> Result<(), ImportError> {
    let current = &parent.commit.target;
    if input.journal.parent != *current {
        return Err(ImportError::WrongParent);
    }
    let height = current
        .height
        .checked_add(1)
        .ok_or(ImportError::WrongHeight)?;
    let next = height.checked_add(1).ok_or(ImportError::WrongHeight)?;
    if input.journal.target_height != height
        || input.execution.header.number != height
        || i64::try_from(height).ok() != Some(input.finalized.header.height)
        || i64::try_from(next).ok() != Some(input.lookahead.frame.header.height)
    {
        return Err(ImportError::WrongHeight);
    }
    Ok(())
}
