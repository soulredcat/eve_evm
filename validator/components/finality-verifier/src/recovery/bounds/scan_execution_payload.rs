// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    scan_execution_payload_with_limit::scan_execution_payload_with_limit,
    types::MAXIMUM_RECOVERY_BYTES,
};
use crate::recovery::RecoveryError;

/// Preserve the compact version's existing enclosing list ceiling.
pub(crate) fn scan_execution_payload(input: &[u8]) -> Result<(), RecoveryError> {
    scan_execution_payload_with_limit(input, MAXIMUM_RECOVERY_BYTES)
}
