// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::StateBudget;
use std::fs::File;

pub const MAXIMUM_CHECKPOINT_CHUNK_BYTES: usize = 4_194_304;
pub(super) const MAGIC: &[u8; 24] = b"EVE_CHECKPOINT_STORE_V1\0";
pub(super) const HEADER_BYTES: usize = 76;
pub(super) const REFERENCE_BYTES: usize = 48;

#[derive(Clone, Copy, Debug)]
pub struct CheckpointLimits {
    pub logical: StateBudget,
    pub maximum_body_bytes: usize,
    pub maximum_chunk_bytes: usize,
    pub maximum_chunks: usize,
    pub maximum_manifest_bytes: usize,
}

#[derive(Debug)]
pub enum CheckpointError {
    Io(std::io::Error),
    State(eve_state::StateError),
    InvalidLimits,
    ResourceReservation,
    InvalidManifest,
    TargetMismatch,
    UnsafeEntry,
    NamespaceOccupied,
    MissingChunk,
    CorruptChunk,
    AlreadyComplete,
    ValidEntry,
    UnsupportedPlatform,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CheckpointChunkStatus {
    Present,
    Missing,
    Corrupt,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CheckpointManifestStats {
    pub manifest_bytes: usize,
    pub body_bytes: usize,
    pub chunk_bytes: usize,
    pub chunks: usize,
    pub target_version_bytes: usize,
    pub body_sha256: [u8; 32],
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CheckpointResumeObservation {
    pub verified_chunks: usize,
    pub missing_chunks: usize,
    pub corrupt_chunks: usize,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct ManifestSummary {
    pub id: [u8; 32],
    pub body_hash: [u8; 32],
    pub body_bytes: usize,
    pub chunk_bytes: usize,
    pub chunks: usize,
    pub version_bytes: usize,
}

#[derive(Debug)]
pub struct CheckpointManifestPreflight<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) summary: ManifestSummary,
    pub(super) limits: CheckpointLimits,
}

pub struct CheckpointTransfer {
    pub(super) directory: File,
    pub(super) manifest: Vec<u8>,
    pub(super) summary: ManifestSummary,
    pub(super) limits: CheckpointLimits,
    pub(super) initial_resume: CheckpointResumeObservation,
}

/// Actual local file integrity and successful sync only; no state, finality or activation authority.
pub struct CompletedCheckpointStore {
    pub(super) transfer: CheckpointTransfer,
}

/// Caller must retain its actual body reservation for this entire owned-buffer lifetime.
pub struct CheckpointBody {
    pub(super) bytes: Vec<u8>,
    pub(super) budget: StateBudget,
}

#[derive(Clone, Copy)]
pub(super) struct ChunkReference {
    pub length: usize,
    pub offset: usize,
    pub hash: [u8; 32],
}
