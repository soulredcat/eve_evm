// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::checkpoints::CheckpointResumeObservation;
use sha2::Sha256;
use std::fs::File;

pub(super) const MAGIC: &[u8; 24] = b"EVE_CHECKPOINT_PROOF_V1\0";
pub(super) const HEADER_BYTES: usize = 144;
pub(super) const REFERENCE_BYTES: usize = 56;
pub(super) const COMPLETION_MAGIC: &[u8; 16] = b"EVE_PROOF_DONE_1";

#[derive(Clone, Copy, Debug)]
pub struct CheckpointProofLimits {
    pub maximum_witness_bytes: usize,
    pub maximum_total_bytes: usize,
    pub maximum_files: usize,
    pub maximum_manifest_bytes: usize,
    pub maximum_disk_bytes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointProofKind {
    Execution,
    ClosingLookahead,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointProofPendingKind {
    Manifest,
    Completion,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointProofReferenceInput {
    pub height: u64,
    pub kind: CheckpointProofKind,
    pub length: usize,
    pub sha256: [u8; 32],
}

#[derive(Clone, Copy, Debug)]
pub(super) struct ProofSummary {
    pub id: [u8; 32],
    pub stream_hash: [u8; 32],
    pub height: u64,
    pub files: usize,
    pub total_bytes: usize,
    pub version_bytes: usize,
}

pub struct CheckpointProofManifestPreflight<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) summary: ProofSummary,
    pub(super) limits: CheckpointProofLimits,
}

/// Caller retains its actual metadata lease for this entire directory/manifest ownership lifetime.
pub struct CheckpointProofTransfer {
    pub(super) directory: File,
    pub(super) manifest: Vec<u8>,
    pub(super) summary: ProofSummary,
    pub(super) limits: CheckpointProofLimits,
    pub(super) initial_resume: CheckpointResumeObservation,
}

/// Complete synced local bytes only; never an execution, consensus or activation capability.
pub struct CompletedCheckpointProofStore {
    pub(super) transfer: CheckpointProofTransfer,
}

/// Caller retains its actual witness-body lease; each value owns only one immutable blob.
pub struct CheckpointProofWitness {
    pub(super) bytes: Vec<u8>,
    pub(super) reference: CheckpointProofReferenceInput,
}

/// Streaming local checksum builder. No signature or protocol validation is performed.
pub struct CheckpointProofStreamHasher {
    pub(super) hash: Sha256,
    pub(super) height: u64,
    pub(super) next: u64,
    pub(super) limits: CheckpointProofLimits,
    pub(super) total: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointProofManifestStats {
    pub files: usize,
    pub height: u64,
    pub total_bytes: usize,
    pub manifest_bytes: usize,
    pub stream_sha256: [u8; 32],
}
