// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::{
    RejectedSegmentedReservation, SegmentedBatchPlan, SegmentedBatchReservation, SegmentedError,
    UnboundSegmentedReservation,
};
use eve_storage::records::segmented::SegmentedMarkerMetadata;

/// Introduce the actual local verified binding without allocating or releasing parts.
pub fn bind_segmented_reservation(
    reservation: UnboundSegmentedReservation,
    target_state_binding: [u8; 32],
) -> Result<SegmentedBatchReservation, Box<RejectedSegmentedReservation>> {
    if target_state_binding == [0; 32] {
        return Err(Box::new(RejectedSegmentedReservation {
            error: SegmentedError::InvalidPlan,
            reservation,
        }));
    }
    Ok(SegmentedBatchReservation {
        plan: SegmentedBatchPlan {
            layout: reservation.layout,
            marker: SegmentedMarkerMetadata {
                identity: reservation.layout.identity,
                target_state_binding,
            },
        },
        id: reservation.id,
        parts: reservation.parts,
        metadata: reservation.metadata,
    })
}
