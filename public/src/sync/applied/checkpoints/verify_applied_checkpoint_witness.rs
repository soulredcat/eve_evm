// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{ChargedCheckpointArtifacts, CheckpointAppliedError};
use crate::sync::applied::resources::reserve_estimated_working;
use eve_finality_verifier::{
    CheckpointSession, CheckpointWitnessWireKind, checkpoint_witness_wire_kind,
    decode_checkpoint_witness_wire, preflight_checkpoint_witness_wire,
    required_checkpoint_witness_decode_reservation, verify_checkpoint_witness,
};
use eve_storage::checkpoints::proofs::{
    CheckpointProofKind, checkpoint_proof_witness_bytes, checkpoint_proof_witness_reference,
    read_checkpoint_proof_witness, required_checkpoint_proof_witness_reservation,
};

/// Read/decode exactly one locally intact blob while all actual body/decoder leases remain alive.
pub(super) fn verify_applied_checkpoint_witness(
    artifacts: &ChargedCheckpointArtifacts,
    session: &mut CheckpointSession,
    height: u64,
    session_reserved: usize,
) -> Result<(), CheckpointAppliedError> {
    let index = usize::try_from(
        height
            .checked_sub(1)
            .ok_or(CheckpointAppliedError::InvalidArtifactBinding)?,
    )
    .map_err(|_| CheckpointAppliedError::InvalidArtifactBinding)?;
    let body_required =
        required_checkpoint_proof_witness_reservation(&artifacts.proof_store, index)
            .map_err(CheckpointAppliedError::Storage)?;
    let _body = reserve_estimated_working(&artifacts.content.working, body_required)
        .map_err(CheckpointAppliedError::Applied)?;
    let _storage = crate::sync::applied::resources::storage_admission::reserve_checkpoint_storage(
        &artifacts.content.storage,
        body_required,
    )
    .map_err(CheckpointAppliedError::Applied)?;
    let blob = read_checkpoint_proof_witness(&artifacts.proof_store, index, body_required)
        .map_err(CheckpointAppliedError::Storage)?;
    let reference = checkpoint_proof_witness_reference(&blob);
    if reference.height != height {
        return Err(CheckpointAppliedError::InvalidArtifactBinding);
    }
    let sealed = preflight_checkpoint_witness_wire(
        checkpoint_proof_witness_bytes(&blob),
        &artifacts.content.limits.content.logical,
        artifacts.content.limits.verification,
    )
    .map_err(CheckpointAppliedError::Witness)?;
    if !matches!(
        (reference.kind, checkpoint_witness_wire_kind(&sealed)),
        (
            CheckpointProofKind::Execution,
            CheckpointWitnessWireKind::Execution
        ) | (
            CheckpointProofKind::ClosingLookahead,
            CheckpointWitnessWireKind::Lookahead
        )
    ) {
        return Err(CheckpointAppliedError::InvalidArtifactBinding);
    }
    let required = required_checkpoint_witness_decode_reservation(&sealed)
        .map_err(CheckpointAppliedError::Witness)?;
    let raw_copies = checkpoint_proof_witness_bytes(&blob)
        .len()
        .checked_mul(6)
        .ok_or(CheckpointAppliedError::Applied(
            crate::sync::applied::AppliedError::ArithmeticOverflow,
        ))?;
    let _codec_staging =
        crate::sync::applied::resources::storage_admission::reserve_snapshot_staging(
            &artifacts.content.storage,
            raw_copies,
        )
        .map_err(crate::sync::applied::resources::storage_admission::map_storage_admission_error)
        .map_err(CheckpointAppliedError::Applied)?;
    let _decoded = reserve_estimated_working(&artifacts.content.working, required)
        .map_err(CheckpointAppliedError::Applied)?;
    let witness = decode_checkpoint_witness_wire(&sealed, required)
        .map_err(CheckpointAppliedError::Witness)?;
    verify_checkpoint_witness(session, &witness, session_reserved)
        .map_err(CheckpointAppliedError::Checkpoint)
}
