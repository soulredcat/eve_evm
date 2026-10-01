// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ApprovalError;
use eve_evm::{BlockExecutionError, CompleteExecutionError};

pub(super) fn classify_execution_error(error: CompleteExecutionError) -> ApprovalError {
    match &error {
        CompleteExecutionError::Execution(
            BlockExecutionError::InvalidTransaction { .. }
            | BlockExecutionError::BlockGasLimit { .. }
            | BlockExecutionError::InvalidEnvironment(_),
        ) => ApprovalError::InvalidExecution(error),
        _ => ApprovalError::Unavailable {
            reason: "canonical execution state/resources unavailable",
            cause: Some(error),
        },
    }
}
