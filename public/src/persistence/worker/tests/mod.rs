// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

mod admission;
mod nonblocking_ack;
mod recovery;
mod resources;

use super::types::WorkerState;
use crate::persistence::handoff::*;
use eve_storage::records::*;
use std::{
    path::PathBuf,
    sync::{
        Arc,
        mpsc::{self, Receiver, SyncSender},
    },
};

pub(super) struct AppendPause {
    pub(super) entered: SyncSender<()>,
    pub(super) resume: Receiver<()>,
}

pub(super) struct Fixture {
    pub(super) _directory: tempfile::TempDir,
    pub(super) path: PathBuf,
    pub(super) identity: OpaqueRecordIdentity,
    pub(super) budget: OpaqueRecordBudget,
    pub(super) repository: OpaqueRecordRepository,
    pub(super) pool: Arc<HandoffPool>,
}

pub(super) fn fixture() -> Fixture {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("opaque");
    let identity = OpaqueRecordIdentity {
        genesis_hash: [1; 32],
        owner: [2; 32],
        domain: [3; 32],
    };
    let mut budget = development_opaque_record_budget();
    // Explicit fixture policy fits the public profile's existing 32-file envelope.
    budget.maximum_open_files = 32;
    let repository = open_opaque_record_repository(&path, identity, budget).unwrap();
    let pool = create_handoff_pool(eve_node_policy::development_public_budget())
        .ok()
        .unwrap();
    Fixture {
        _directory: directory,
        path,
        identity,
        budget,
        repository,
        pool,
    }
}

pub(super) fn payload(pool: &Arc<HandoffPool>, bytes: &[u8]) -> RecoveryPayload {
    let mut reservation = reserve_recovery_payload(pool, bytes.len()).ok().unwrap();
    write_reserved_payload(&mut reservation, bytes).unwrap();
    seal_recovery_payload(reservation).ok().unwrap()
}

pub(super) fn install_pause(worker: &super::RecordWorker) -> (Receiver<()>, SyncSender<()>) {
    let (entered, entered_receiver) = mpsc::sync_channel(1);
    let (resume_sender, resume) = mpsc::sync_channel(1);
    *worker.state.pause.lock().unwrap() = Some(AppendPause { entered, resume });
    (entered_receiver, resume_sender)
}

/// Test-only pause before actual repository append, not an injected fsync/power-loss claim.
pub(super) fn pause_before_append(state: &WorkerState) {
    let pause = state.pause.lock().unwrap().take();
    if let Some(pause) = pause {
        let _ = pause.entered.send(());
        let _ = pause.resume.recv();
    }
}
