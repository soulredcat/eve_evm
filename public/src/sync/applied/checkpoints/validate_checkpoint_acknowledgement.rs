// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{CheckpointAppliedError, types::PendingCheckpointActivation};
use crate::{
    persistence::segmented::checkpoints::{
        CheckpointAck, checkpoint_record_cursor, checkpoint_record_metadata,
    },
    sync::applied::AppliedOwner,
};

pub(super) fn validate_checkpoint_acknowledgement(
    owner: &AppliedOwner,
    pending: &PendingCheckpointActivation,
    ack: CheckpointAck,
) -> Result<(), CheckpointAppliedError> {
    let metadata = checkpoint_record_metadata(&pending.record);
    if ack.metadata != metadata
        || ack.cursor != checkpoint_record_cursor(&pending.record)
        || ack.cursor != pending.publication.durable_cursor
        || metadata.previous_opaque_cursor != owner.admitted_cursor
        || metadata.previous_logical_anchor.cursor != owner.durable_cursor
        || ack.database_sequence <= owner.database_sequence
        || ack.cursor.sequence
            != owner
                .admitted_cursor
                .sequence
                .checked_add(1)
                .ok_or(CheckpointAppliedError::InvalidAcknowledgement)?
    {
        return Err(CheckpointAppliedError::InvalidAcknowledgement);
    }
    Ok(())
}
