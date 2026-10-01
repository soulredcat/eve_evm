// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::ApprovalError;

/// Preserve typed local failure detail for the owning actor, never in a peer response.
pub(in crate::consensus) fn approval_error_diagnostic(error: ApprovalError) -> anyhow::Error {
    match error {
        ApprovalError::InvalidExecution(cause) => {
            anyhow::anyhow!("invalid proposal execution: {cause:?}")
        }
        ApprovalError::Unavailable {
            reason,
            cause: Some(cause),
        } => anyhow::anyhow!("{reason}: {cause:?}"),
        ApprovalError::Unavailable {
            reason,
            cause: None,
        } => anyhow::anyhow!(reason),
    }
}
