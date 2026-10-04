// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::*;
use crate::sync::applied::{
    AppliedOwner, applied_owner_reader, reserve_applied_working,
    tests::import_fixtures::ImportChain,
};
use eve_finality_verifier::{
    CheckpointExecutionWitness, CheckpointWitness, encode_checkpoint_witness_wire,
};
use eve_state::{StateCommit, encode_state_commit, encode_state_version, preflight_state_commit};
use eve_storage::checkpoints::{
    create_checkpoint_manifest,
    proofs::{
        CheckpointProofKind, CheckpointProofReferenceInput, begin_checkpoint_proof_stream_hash,
        create_checkpoint_proof_manifest, finish_checkpoint_proof_stream_hash,
        required_checkpoint_proof_metadata_reservation, update_checkpoint_proof_stream_hash,
    },
};
use sha2::{Digest, Sha256};
use std::fs::File;

pub(crate) fn limits() -> AppliedCheckpointLimits {
    AppliedCheckpointLimits {
        content: eve_storage::checkpoints::CheckpointLimits {
            logical: eve_state::development_state_budget(),
            maximum_body_bytes: 4 * 1_048_576,
            maximum_chunk_bytes: 1_024,
            maximum_chunks: 4_096,
            maximum_manifest_bytes: 262_144,
        },
        proofs: eve_storage::checkpoints::proofs::CheckpointProofLimits {
            maximum_witness_bytes: 65_536,
            maximum_total_bytes: 1_048_576,
            maximum_files: 16,
            maximum_manifest_bytes: 65_536,
            maximum_disk_bytes: 2 * 1_048_576,
        },
        verification: eve_finality_verifier::CheckpointLimits {
            maximum_height_gap: 10,
            maximum_witness_bytes: 65_536,
        },
    }
}

pub(crate) fn witnesses(chain: &ImportChain, height: usize) -> Vec<CheckpointWitness> {
    let mut witnesses: Vec<_> = (0..height)
        .map(|index| {
            CheckpointWitness::Execution(Box::new(CheckpointExecutionWitness {
                native: eve_finality_verifier::NativeDataFrame {
                    frame: chain.inputs[index].finalized.clone(),
                    transactions: chain.commits[index + 1].block.transactions.clone(),
                },
                version: chain.commits[index + 1].target.clone(),
                block: chain.commits[index + 1].block.clone(),
            }))
        })
        .collect();
    witnesses.push(CheckpointWitness::Lookahead(Box::new(
        chain.inputs[height - 1].lookahead.clone(),
    )));
    witnesses
}

pub(crate) fn stage_artifacts(
    owner: &AppliedOwner,
    base: &File,
    target: &StateCommit,
    witnesses: &[CheckpointWitness],
) -> Result<ChargedCheckpointArtifacts, CheckpointAppliedError> {
    let limits = limits();
    let reader = applied_owner_reader(owner);
    let _ingress =
        reserve_applied_working(&reader, 8 * 1_048_576).map_err(CheckpointAppliedError::Applied)?;
    let bytes = encode_state_commit(target, &limits.content.logical)
        .map_err(CheckpointAppliedError::State)?;
    let sealed = preflight_state_commit(&bytes, &limits.content.logical)
        .map_err(CheckpointAppliedError::State)?;
    let metadata =
        eve_storage::checkpoints::required_checkpoint_metadata_reservation(&limits.content)
            .map_err(CheckpointAppliedError::Storage)?;
    let manifest = create_checkpoint_manifest(&sealed, &target.target, &limits.content, metadata)
        .map_err(CheckpointAppliedError::Storage)?;
    let target_encoding =
        encode_state_version(&target.target).map_err(CheckpointAppliedError::State)?;
    let mut transfer =
        begin_applied_checkpoint_transfer(owner, base, &manifest, &target_encoding, limits)?;
    for (index, chunk) in bytes.chunks(limits.content.maximum_chunk_bytes).enumerate() {
        write_applied_checkpoint_chunk(&mut transfer, index, chunk)?;
    }
    let content = complete_applied_checkpoint_transfer(transfer)?;
    let proof_metadata = required_checkpoint_proof_metadata_reservation(&limits.proofs)
        .map_err(CheckpointAppliedError::Storage)?;
    let mut stream = begin_checkpoint_proof_stream_hash(
        &content.content_id,
        &content.content.body_sha256,
        &target.target,
        &limits.proofs,
        proof_metadata,
    )
    .map_err(CheckpointAppliedError::Storage)?;
    let mut references = Vec::new();
    let mut blobs = Vec::new();
    for (index, witness) in witnesses.iter().enumerate() {
        let bytes =
            encode_checkpoint_witness_wire(witness, &limits.content.logical, limits.verification)
                .map_err(CheckpointAppliedError::Witness)?;
        let reference = CheckpointProofReferenceInput {
            height: index as u64 + 1,
            kind: if index + 1 == witnesses.len() {
                CheckpointProofKind::ClosingLookahead
            } else {
                CheckpointProofKind::Execution
            },
            length: bytes.len(),
            sha256: Sha256::digest(&bytes).into(),
        };
        update_checkpoint_proof_stream_hash(&mut stream, &reference, &bytes)
            .map_err(CheckpointAppliedError::Storage)?;
        references.push(reference);
        blobs.push(bytes);
    }
    let hash =
        finish_checkpoint_proof_stream_hash(stream).map_err(CheckpointAppliedError::Storage)?;
    let proof_manifest = create_checkpoint_proof_manifest(
        &content.content_id,
        &content.content.body_sha256,
        &target.target,
        &references,
        &hash,
        &limits.proofs,
        proof_metadata,
    )
    .map_err(CheckpointAppliedError::Storage)?;
    let mut proofs = begin_applied_checkpoint_proofs(content, base, &proof_manifest)?;
    for (index, blob) in blobs.iter().enumerate() {
        write_applied_checkpoint_witness(&mut proofs, index, blob)?;
    }
    complete_applied_checkpoint_proofs(proofs)
}

pub(super) fn advance_first_block(owner: &mut AppliedOwner, chain: &ImportChain) {
    crate::sync::applied::try_apply_recovery_bytes(owner, &chain.records[0]).unwrap();
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(10);
    while crate::sync::applied::poll_applied_durability(owner)
        .unwrap()
        .durable_recovery
        .0
        != 1
    {
        assert!(std::time::Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
