// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::DeltaServingError;

/// Conservative 128-byte allowance per serialized byte covers declared decoded
/// map/root/projection work plus control scratch. It is operator admission only;
/// actual allocator/CPU/IO interference and larger-state suitability need measurement.
pub(super) fn estimate_delta_commit_charge(
    encoded_bytes: usize,
) -> Result<usize, DeltaServingError> {
    encoded_bytes
        .checked_mul(128)
        .and_then(|bytes| bytes.checked_add(2 * 1_048_576))
        .ok_or(DeltaServingError::ResourceLimit)
}
