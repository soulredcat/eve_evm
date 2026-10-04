// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_state::{
    Bytes, StateCommit, development_state_budget, encode_state_commit, encode_state_version,
    initialize_development_state, preflight_state_commit,
};
use eve_storage::checkpoints::*;
use std::{fs::File, os::unix::fs::PermissionsExt, path::PathBuf};

pub struct Fixture {
    pub commit: StateCommit,
    pub body: Vec<u8>,
    pub version: Bytes,
    pub manifest: Vec<u8>,
    pub limits: CheckpointLimits,
    pub metadata: usize,
    pub io: usize,
}

pub fn fixture() -> Fixture {
    let budget = development_state_budget();
    let genesis = eve_development_fixtures::genesis::genesis();
    let commit = initialize_development_state(&genesis, &budget).unwrap();
    let body = encode_state_commit(&commit, &budget).unwrap();
    let version = encode_state_version(&commit.target).unwrap();
    let limits = CheckpointLimits {
        logical: budget,
        maximum_body_bytes: body.len(),
        maximum_chunk_bytes: 512,
        maximum_chunks: 64,
        maximum_manifest_bytes: 4_096,
    };
    let metadata = required_checkpoint_metadata_reservation(&limits).unwrap();
    let io = required_checkpoint_io_reservation(&limits).unwrap();
    let preflight = preflight_state_commit(&body, &budget).unwrap();
    let manifest =
        create_checkpoint_manifest(&preflight, &commit.target, &limits, metadata).unwrap();
    Fixture {
        commit,
        body,
        version,
        manifest,
        limits,
        metadata,
        io,
    }
}

pub fn temporary_root() -> (tempfile::TempDir, File) {
    let directory = tempfile::Builder::new()
        .prefix("eve-checkpoint-")
        .tempdir_in("/tmp")
        .unwrap();
    let root = File::open(directory.path()).unwrap();
    (directory, root)
}

pub fn manifest(fixture: &Fixture) -> CheckpointManifestPreflight<'_> {
    preflight_checkpoint_manifest(&fixture.manifest, &fixture.version, &fixture.limits).unwrap()
}

pub fn namespace(directory: &tempfile::TempDir, fixture: &Fixture) -> PathBuf {
    let id = checkpoint_manifest_id(&manifest(fixture));
    let name: String = id.iter().map(|byte| format!("{byte:02x}")).collect();
    directory.path().join(name)
}

pub fn chunk_path(directory: &tempfile::TempDir, fixture: &Fixture, index: usize) -> PathBuf {
    namespace(directory, fixture).join(format!("chunk-{index:04}.bin"))
}

pub fn write_all(transfer: &mut CheckpointTransfer, fixture: &Fixture) {
    for (index, chunk) in fixture
        .body
        .chunks(fixture.limits.maximum_chunk_bytes)
        .enumerate()
    {
        write_checkpoint_chunk(transfer, index, chunk, fixture.io).unwrap();
    }
}

pub fn create_namespace(directory: &tempfile::TempDir, fixture: &Fixture) {
    let path = namespace(directory, fixture);
    std::fs::create_dir(&path).unwrap();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).unwrap();
}

pub fn corrupt_chunk(directory: &tempfile::TempDir, fixture: &Fixture, index: usize) {
    let path = chunk_path(directory, fixture, index);
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
    let mut bytes = std::fs::read(&path).unwrap();
    bytes[0] ^= 1;
    std::fs::write(path, bytes).unwrap();
}
