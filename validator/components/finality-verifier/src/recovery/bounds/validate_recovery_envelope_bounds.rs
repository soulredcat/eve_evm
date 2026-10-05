// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::measure_recovery_envelope_bytes::measure_recovery_envelope_bytes;
use crate::recovery::{CompactRecoveryEnvelopeV1, RecoveryError};

/// Operator payload admission only; it creates no finality or execution capability.
pub fn validate_recovery_envelope_bounds(
    envelope: &CompactRecoveryEnvelopeV1,
) -> Result<(), RecoveryError> {
    measure_recovery_envelope_bytes(envelope).map(|_| ())
}
