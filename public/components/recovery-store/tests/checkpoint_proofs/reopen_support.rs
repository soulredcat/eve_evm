// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::support::*;
use eve_storage::checkpoints::proofs::*;
use std::path::Path;

pub fn expected(fixture: &Fixture) -> CheckpointProofStoreIdentity {
    CheckpointProofStoreIdentity {
        manifest_id: checkpoint_proof_manifest_id(&manifest(fixture)),
        snapshot_manifest_id: fixture.snapshot,
        snapshot_body_sha256: fixture.body,
        stream_sha256: checkpoint_proof_manifest_stats(&manifest(fixture)).stream_sha256,
    }
}

pub fn captured_files(path: &Path) -> Vec<(String, Vec<u8>)> {
    let mut files: Vec<_> = std::fs::read_dir(path)
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            (
                entry.file_name().to_str().unwrap().to_owned(),
                std::fs::read(entry.path()).unwrap(),
            )
        })
        .collect();
    files.sort_by(|left, right| left.0.cmp(&right.0));
    files
}
