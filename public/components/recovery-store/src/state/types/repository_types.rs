// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::DurableStateAck;
use crate::state::StateStorageBudget;
use eve_state::{StateCommit, StateIdentity};
use rocksdb::DB;
use std::sync::{
    Arc, RwLock,
    atomic::{AtomicBool, AtomicUsize},
};

/// One private DB object and exclusive write owner. Reads have no write capability.
pub struct StateRepository {
    pub(crate) database: Arc<DB>,
    pub(crate) identity: StateIdentity,
    pub(crate) genesis: Arc<StateCommit>,
    pub(crate) budget: StateStorageBudget,
    pub(crate) fence: Arc<AtomicBool>,
    pub(crate) snapshots: Arc<AtomicUsize>,
    pub(crate) publication: Arc<RwLock<Option<DurableStateAck>>>,
    #[cfg(test)]
    pub(crate) simulated_failure: Option<SimulatedCommitFailure>,
    #[cfg(test)]
    pub(crate) simulated_index_failure: Option<SimulatedCommitFailure>,
}

#[cfg(test)]
#[derive(Clone, Copy)]
pub(crate) enum SimulatedCommitFailure {
    BeforeWrite,
    AfterSuccessfulSync,
}

pub struct StateReader {
    pub(crate) database: Arc<DB>,
    pub(crate) identity: StateIdentity,
    pub(crate) genesis: Arc<StateCommit>,
    pub(crate) budget: StateStorageBudget,
    pub(crate) fence: Arc<AtomicBool>,
    pub(crate) snapshots: Arc<AtomicUsize>,
    pub(crate) publication: Arc<RwLock<Option<DurableStateAck>>>,
}
