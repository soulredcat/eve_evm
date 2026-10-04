// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    required_segmented_recovery_reservation::required_segmented_recovery_reservation,
    types::{
        RecoveredSegmentedBundle, RejectedSegmentedRecovery, SegmentedRecoveryError,
        SegmentedRecoveryScan,
    },
};
use crate::records::segmented::SegmentedRecoveryAnchor;

/// Caller-confirmed progress only: public MUST verify V2/H+1 and the exact target
/// binding first. This storage operation performs no finality verification.
/// Return the same leased Vec for reuse; failures preserve the bundle and parent.
pub fn accept_segmented_recovery(
    scan: &mut SegmentedRecoveryScan,
    mut bundle: Box<RecoveredSegmentedBundle>,
    caller_confirmed: SegmentedRecoveryAnchor,
    reserved_bytes: usize,
) -> Result<(), RejectedSegmentedRecovery> {
    let required = required_segmented_recovery_reservation(&scan.budget, &scan.limits);
    let error = if scan.failed {
        Some(SegmentedRecoveryError::FailedScanner)
    } else if required.is_err() {
        required.err()
    } else if reserved_bytes < required.unwrap_or(usize::MAX) {
        Some(SegmentedRecoveryError::ReservationTooSmall)
    } else if bundle.namespace != scan.namespace
        || scan.pending != Some(bundle.completion)
        || caller_confirmed != bundle.completion.target
        || scan.physical != caller_confirmed.cursor
        || bundle.body.capacity() != scan.limits.maximum_logical_bytes
    {
        Some(SegmentedRecoveryError::WrongConfirmation)
    } else {
        None
    };
    if let Some(error) = error {
        return Err(RejectedSegmentedRecovery { error, bundle });
    }
    bundle.body.clear();
    scan.body = bundle.body;
    scan.anchor = caller_confirmed;
    scan.pending = None;
    Ok(())
}
