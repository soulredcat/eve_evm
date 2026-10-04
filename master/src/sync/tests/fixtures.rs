// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::recovery_support::{RecoveryChain, envelope, recovery_chain};
use crate::sync::{MasterSyncConfig, master_sync_development_config};
use eve_finality_verifier::{AuthenticatedImportInput, encode_logical_import_wire};
use eve_state::project_state_journal;
use std::{path::Path, process::Command};

pub(super) struct Fixture {
    pub root: tempfile::TempDir,
    pub chain: RecoveryChain,
    pub config: MasterSyncConfig,
    pub wires: Vec<Vec<u8>>,
}

pub(super) fn fixture() -> Fixture {
    let root = tempfile::tempdir().unwrap();
    assert!(
        Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(root.path())
            .status()
            .unwrap()
            .success()
    );
    std::fs::write(root.path().join(".gitignore"), "/local-tests/\n").unwrap();
    let chain = recovery_chain();
    let mut config = master_sync_development_config(
        root.path().canonicalize().unwrap(),
        Path::new("local-tests/master-sync").to_owned(),
    );
    config.acknowledge_unsafe_development = true;
    config.maximum_proof_bytes = 65_536;
    let wires = (1..=2)
        .map(|height| {
            encode_logical_import_wire(
                &import_input(&chain, height, &config),
                &config.storage.logical,
            )
            .unwrap()
        })
        .collect();
    Fixture {
        root,
        chain,
        config,
        wires,
    }
}

pub(super) fn import_input(
    chain: &RecoveryChain,
    height: usize,
    config: &MasterSyncConfig,
) -> AuthenticatedImportInput {
    let envelope = envelope(chain, height);
    AuthenticatedImportInput {
        journal: project_state_journal(
            &chain.commits[height - 1].state,
            &chain.commits[height - 1].target,
            &chain.commits[height].state,
            height as u64,
            &config.storage.logical,
        )
        .unwrap(),
        execution: envelope.execution,
        finalized: envelope.finalized,
        lookahead: envelope.lookahead,
    }
}

pub(super) fn proof_directory(fixture: &Fixture) -> std::path::PathBuf {
    fixture
        .root
        .path()
        .join(&fixture.config.data)
        .join("proofs")
}
