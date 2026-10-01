// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    approval::ExecutionApproval, signing::SignerConfig,
    transport::proposals::VerifiedLocalEngineProposal,
};
use eve_state::StateVersion;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

pub(in crate::consensus) struct ApprovalRegistry {
    pub(super) state: Mutex<RegistryState>,
}

#[derive(Default)]
pub(super) struct RegistryState {
    pub config: Option<SignerConfig>,
    pub parent: Option<StateVersion>,
    pub sequence: u64,
    pub full: BTreeMap<[u8; 32], Arc<ExecutionApproval>>,
    pub raw: BTreeMap<[u8; 32], Arc<RetainedInput>>,
    pub current: Option<[u8; 32]>,
    pub prevote_pin: Option<[u8; 32]>,
    pub precommit_pin: Option<[u8; 32]>,
    pub insertion: u64,
}

/// Original non-Clone token stays encapsulated; callers cannot replay it as fresh unlocked evidence.
pub(super) struct RetainedInput {
    pub source: VerifiedLocalEngineProposal,
    pub bytes: usize,
    pub insertion: u64,
}

pub(super) const MAXIMUM_FULL_APPROVALS: usize = 2;
pub(super) const MAXIMUM_RAW_INPUTS: usize = 4;
pub(super) const MAXIMUM_RAW_BYTES: usize = 16 * 1024 * 1024;

pub(in crate::consensus) struct ApprovalRegistryStatus {
    pub full: usize,
    pub raw: usize,
    pub retained_encoded_bytes: usize,
    pub current: Option<[u8; 32]>,
    pub prevote_pin: Option<[u8; 32]>,
    pub precommit_pin: Option<[u8; 32]>,
}
