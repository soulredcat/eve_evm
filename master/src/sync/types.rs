// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use eve_finality_verifier::ImportedState;
use eve_state::{B256, StateVersion};
use eve_storage::state::{StateReader, StateRepository, StateStorageBudget};
use serde::Serialize;
use std::{fs::File, path::PathBuf, sync::Arc};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

#[derive(Clone, Debug)]
pub struct MasterSyncConfig {
    pub root: PathBuf,
    pub data: PathBuf,
    pub mode: String,
    pub acknowledge_unsafe_development: bool,
    pub storage: StateStorageBudget,
    pub working_bytes: usize,
    pub maximum_proof_bytes: usize,
    pub maximum_proof_files: u64,
    pub maximum_archive_bytes: u64,
    pub maximum_retained_commit_bytes: u64,
}

pub struct MasterFollower {
    pub(super) current: RetainedImport,
    pub(super) repository: StateRepository,
    pub(super) reader: StateReader,
    pub(super) directory: File,
    pub(super) config: MasterSyncConfig,
    pub(super) proof_count: u64,
    pub(super) archive_bytes: u64,
    pub(super) retained_commit_bytes: u64,
    pub(super) rejected_staging: bool,
    pub(super) fenced: bool,
    pub(super) pool: Arc<Semaphore>,
    #[cfg(test)]
    pub(super) fault: Option<MasterSyncFault>,
    // Keep storage/opening allowances until every repository/reader allocation drops.
    pub(super) _archive_lease: OwnedSemaphorePermit,
}

pub(super) struct RetainedImport {
    pub state: Arc<ImportedState>,
    pub _lease: OwnedSemaphorePermit,
}

pub(super) struct ChargedProof {
    pub bytes: Vec<u8>,
    pub _lease: OwnedSemaphorePermit,
}

pub(super) struct ArchiveInventory {
    pub completed: u64,
    pub bytes: u64,
    pub staging: bool,
    pub rejected: bool,
}

pub(super) enum PreparationFailure {
    Resource(anyhow::Error),
    Invalid(anyhow::Error),
}

#[derive(Debug, Serialize)]
pub struct MasterSyncStatus {
    pub role: &'static str,
    pub security_profile: &'static str,
    pub verification_mode: &'static str,
    pub finalized_height: u64,
    pub applied_height: u64,
    pub durable_height: u64,
    pub authenticated_height: u64,
    pub authenticated_validator_finality: bool,
    #[serde(skip_serializing)]
    pub target: StateVersion,
    pub evm_root: B256,
    pub system_root: B256,
    pub execution_hash: B256,
    pub content_digest: B256,
    pub application_commitment: Option<B256>,
    pub genesis_hash: B256,
    pub retained_proof_files: u64,
    pub retained_archive_bytes: u64,
    pub retained_canonical_commit_bytes: u64,
    pub rejected_staging_retained: bool,
    pub fenced: bool,
    pub storage_outcome_unknown: bool,
    pub lag: Option<u64>,
    pub peer_head: Option<u64>,
    pub ready: bool,
}

#[cfg(test)]
#[derive(Clone, Copy, PartialEq)]
pub(super) enum MasterSyncFault {
    PartialStaging,
    AfterProofSync,
    AfterStateSync,
}
