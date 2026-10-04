// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    CheckpointActivationOutcome, CheckpointAppliedError,
    fence_checkpoint_activation::fence_checkpoint_activation,
    validate_checkpoint_acknowledgement::validate_checkpoint_acknowledgement,
};
use crate::{
    persistence::segmented::checkpoints::try_receive_checkpoint_ack,
    sync::applied::{AppliedError, AppliedOwner},
};
use std::sync::Arc;

/// Receive the real ACK outside the RAM guard, then make one conditional immutable publication.
pub fn poll_applied_checkpoint_activation(
    owner: &mut AppliedOwner,
) -> Result<Option<CheckpointActivationOutcome>, CheckpointAppliedError> {
    if owner.checkpoint.is_none() {
        return Ok(None);
    }
    if owner.storage_failed {
        return Err(CheckpointAppliedError::Applied(AppliedError::StorageFailed));
    }
    let pending = owner
        .checkpoint
        .as_mut()
        .ok_or(CheckpointAppliedError::PendingDurability)?;
    if pending.acknowledged.is_none() {
        let ticket = pending
            .ticket
            .as_ref()
            .ok_or(CheckpointAppliedError::InvalidAcknowledgement)?;
        let ack = match try_receive_checkpoint_ack(ticket) {
            Ok(None) => return Ok(None),
            Ok(Some(ack)) => ack,
            Err(error) => {
                fence_checkpoint_activation(owner);
                return Err(CheckpointAppliedError::Segmented(error));
            }
        };
        pending.acknowledged = Some(ack);
    }
    let pending = owner
        .checkpoint
        .as_ref()
        .ok_or(CheckpointAppliedError::PendingDurability)?;
    let ack = pending
        .acknowledged
        .ok_or(CheckpointAppliedError::InvalidAcknowledgement)?;
    if let Err(error) = validate_checkpoint_acknowledgement(owner, pending, ack) {
        fence_checkpoint_activation(owner);
        return Err(error);
    }
    let next = Arc::clone(&pending.publication);
    let parent = Arc::clone(&pending.prepared.artifacts.content.parent);
    let mut publication = match owner.reader.publication.write() {
        Ok(guard) => guard,
        Err(_) => {
            owner.storage_failed = true;
            return Err(CheckpointAppliedError::Applied(
                AppliedError::PublicationUnavailable,
            ));
        }
    };
    if !Arc::ptr_eq(&publication, &parent) {
        drop(publication);
        fence_checkpoint_activation(owner);
        return Err(CheckpointAppliedError::StaleParent);
    }
    let previous = std::mem::replace(&mut *publication, next);
    owner.admitted_cursor = ack.cursor;
    owner.durable_cursor = ack.cursor;
    owner.database_sequence = ack.database_sequence;
    let retired = owner.checkpoint.take();
    drop(publication);
    drop(previous);
    drop(retired);
    Ok(Some(CheckpointActivationOutcome {
        height: ack.metadata.target_height,
        durable_cursor: ack.cursor,
    }))
}
