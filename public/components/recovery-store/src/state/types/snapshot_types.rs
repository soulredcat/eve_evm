// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::state::StateStorageBudget;
use eve_state::{StateCommit, StateVersion};
use rocksdb::{DB, SnapshotWithThreadMode};
use std::sync::{Arc, atomic::AtomicUsize};

pub struct StateSnapshot<'a> {
    pub(crate) snapshot: SnapshotWithThreadMode<'a, DB>,
    pub(crate) genesis: Arc<StateCommit>,
    pub(crate) budget: StateStorageBudget,
    pub(crate) leases: Arc<AtomicUsize>,
    pub(crate) version: StateVersion,
    pub(crate) database_sequence: u64,
}
