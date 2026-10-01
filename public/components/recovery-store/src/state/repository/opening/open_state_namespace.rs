// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    initialize_state_namespace::initialize_state_namespace,
    open_state_database::open_state_database,
};
use crate::state::{
    StateRepository, StateStorageBudget,
    budget::validate_state_storage_budget,
    repository::validation::validate_reopened_repository,
    types::{ACTIVE_KEY, SCHEMA_BYTES, SCHEMA_KEY},
};
use anyhow::{Result, ensure};
use eve_state::StateCommit;
use std::{
    path::Path,
    sync::{
        Arc, RwLock,
        atomic::{AtomicBool, AtomicUsize},
    },
};

pub(crate) fn open_state_namespace(
    path: &Path,
    genesis: &StateCommit,
    budget: StateStorageBudget,
    active: bool,
) -> Result<StateRepository> {
    validate_state_storage_budget(&budget)?;
    let database = open_state_database(path, &budget)?;
    if let Some(schema) = database.get(SCHEMA_KEY)? {
        ensure!(schema == SCHEMA_BYTES, "unsupported full-state schema");
        ensure!(
            database.get(ACTIVE_KEY)?.as_deref() == Some([u8::from(active)].as_slice()),
            "full-state namespace activation state mismatch/incomplete import"
        );
    } else {
        initialize_state_namespace(&database, genesis, &budget, active)?;
    }
    let store = StateRepository {
        database: Arc::new(database),
        identity: genesis.target.identity.clone(),
        genesis: Arc::new(genesis.clone()),
        budget,
        fence: Arc::new(AtomicBool::new(false)),
        snapshots: Arc::new(AtomicUsize::new(0)),
        publication: Arc::new(RwLock::new(None)),
        #[cfg(test)]
        simulated_failure: None,
        #[cfg(test)]
        simulated_index_failure: None,
    };
    let ack = validate_reopened_repository(&store)?;
    *store
        .publication
        .write()
        .map_err(|_| anyhow::anyhow!("durable publication lock poisoned"))? = Some(ack);
    Ok(store)
}
