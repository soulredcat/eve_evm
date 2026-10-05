// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{ExecutionBlockHash, JournalOperation};

use super::{AuthenticatedImportInput, ImportError};

/// Preserve the parent's entire execution-hash map. The sole permitted addition
/// is exactly H/hash(header_H); malformed peer entries are never normalized away.
pub(super) fn validate_import_execution_history(
    input: &AuthenticatedImportInput,
) -> Result<(), ImportError> {
    let current = ExecutionBlockHash(input.execution.header.hash_slow());
    let mut entries = 0_usize;
    for operation in &input.journal.operations {
        if let JournalOperation::SetExecutionBlockHash { height, hash } = operation {
            entries += 1;
            if entries > 1 || *height != input.journal.target_height || *hash != current {
                return Err(ImportError::InvalidExecutionHistory);
            }
        }
    }
    if entries != 1 {
        return Err(ImportError::InvalidExecutionHistory);
    }
    Ok(())
}
