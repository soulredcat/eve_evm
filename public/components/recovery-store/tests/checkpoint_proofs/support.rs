// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{Bytes, StateVersion};
use eve_storage::checkpoints::proofs::*;
use sha2::{Digest, Sha256};
use std::{fs::File, os::unix::fs::PermissionsExt, path::PathBuf};

pub struct Fixture {
    pub target: StateVersion,
    pub version: Bytes,
    pub blobs: Vec<Vec<u8>>,
    pub references: Vec<CheckpointProofReferenceInput>,
    pub manifest: Vec<u8>,
    pub limits: CheckpointProofLimits,
    pub metadata: usize,
    pub io: usize,
    pub snapshot: [u8; 32],
    pub body: [u8; 32],
}

pub fn fixture() -> Fixture {
    let chain = eve_development_fixtures::recovery::recovery_chain();
    let target = chain.commits[2].target.clone();
    let version = eve_state::encode_state_version(&target).unwrap();
    // Opaque structural file fixtures deliberately do not claim serialized consensus witnesses.
    let blobs = vec![vec![0x31; 128], vec![0x42; 257], vec![0x53; 1_024]];
    let references: Vec<_> = blobs
        .iter()
        .enumerate()
        .map(|(index, blob)| CheckpointProofReferenceInput {
            height: index as u64 + 1,
            kind: if index == 2 {
                CheckpointProofKind::ClosingLookahead
            } else {
                CheckpointProofKind::Execution
            },
            length: blob.len(),
            sha256: Sha256::digest(blob).into(),
        })
        .collect();
    let limits = CheckpointProofLimits {
        maximum_witness_bytes: 1_024,
        maximum_total_bytes: 4_096,
        maximum_files: 3,
        maximum_manifest_bytes: 4_096,
        maximum_disk_bytes: 65_536,
    };
    let metadata = required_checkpoint_proof_metadata_reservation(&limits).unwrap();
    let io = required_checkpoint_proof_io_reservation(&limits).unwrap();
    let snapshot = [0x81; 32];
    let body = [0x92; 32];
    let mut stream =
        begin_checkpoint_proof_stream_hash(&snapshot, &body, &target, &limits, metadata).unwrap();
    for (reference, blob) in references.iter().zip(&blobs) {
        update_checkpoint_proof_stream_hash(&mut stream, reference, blob).unwrap();
    }
    let digest = finish_checkpoint_proof_stream_hash(stream).unwrap();
    let manifest = create_checkpoint_proof_manifest(
        &snapshot,
        &body,
        &target,
        &references,
        &digest,
        &limits,
        metadata,
    )
    .unwrap();
    Fixture {
        target,
        version,
        blobs,
        references,
        manifest,
        limits,
        metadata,
        io,
        snapshot,
        body,
    }
}

pub fn manifest(fixture: &Fixture) -> CheckpointProofManifestPreflight<'_> {
    preflight_checkpoint_proof_manifest(
        &fixture.manifest,
        fixture.target.height,
        &fixture.version,
        &fixture.snapshot,
        &fixture.body,
        &fixture.limits,
    )
    .unwrap()
}

pub fn temporary_root() -> (tempfile::TempDir, File) {
    let directory = tempfile::Builder::new()
        .prefix("eve-checkpoint-proofs-")
        .tempdir_in("/tmp")
        .unwrap();
    let root = File::open(directory.path()).unwrap();
    (directory, root)
}

pub fn namespace(directory: &tempfile::TempDir, fixture: &Fixture) -> PathBuf {
    let id = checkpoint_proof_manifest_id(&manifest(fixture));
    let name: String = id.iter().map(|byte| format!("{byte:02x}")).collect();
    directory.path().join(name)
}

pub fn witness_path(directory: &tempfile::TempDir, fixture: &Fixture, index: usize) -> PathBuf {
    namespace(directory, fixture).join(format!("witness-{index:05}.bin"))
}

pub fn write_all(transfer: &mut CheckpointProofTransfer, fixture: &Fixture) {
    for (index, bytes) in fixture.blobs.iter().enumerate() {
        write_checkpoint_proof_witness(transfer, index, bytes, fixture.io).unwrap();
    }
}

pub fn create_namespace(directory: &tempfile::TempDir, fixture: &Fixture) {
    let path = namespace(directory, fixture);
    std::fs::create_dir(&path).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).unwrap();
}

pub fn corrupt_witness(directory: &tempfile::TempDir, fixture: &Fixture, index: usize) {
    let path = witness_path(directory, fixture, index);
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    let mut bytes = std::fs::read(&path).unwrap();
    bytes[0] ^= 1;
    std::fs::write(path, bytes).unwrap();
}
