// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointError, CompletedCheckpointStore, required_checkpoint_io_reservation};

pub fn required_checkpoint_body_reservation(
    store: &CompletedCheckpointStore,
) -> Result<usize, CheckpointError> {
    store
        .transfer
        .summary
        .body_bytes
        .checked_add(required_checkpoint_io_reservation(&store.transfer.limits)?)
        .ok_or(CheckpointError::ResourceReservation)
}
