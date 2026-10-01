// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    initialize_application_history::initialize_application_history,
    validate_application_config::validate_application_config,
};
use crate::consensus::{
    application::{ApplicationConfig, ConsensusApplication},
    approval::create_approval_registry,
    signing::{DurableSigner, signer_status},
};
use anyhow::{Result, ensure};
use eve_storage::{
    records::{OpaqueRecordRepository, opaque_record_cursor},
    state::{StateRepository, StateService, read_state_service},
};
use std::sync::{Arc, Mutex};

pub(in crate::consensus) fn open_application(
    mut repository: StateRepository,
    service: Arc<StateService>,
    replay_repository: OpaqueRecordRepository,
    signer: Arc<Mutex<DurableSigner>>,
    config: ApplicationConfig,
) -> Result<ConsensusApplication> {
    validate_application_config(&config)?;
    initialize_application_history(&mut repository, config.logical_budget.maximum_commit_bytes)?;
    let current = read_state_service(&service)?;
    ensure!(
        current.commit().target.identity == config.genesis.target.identity,
        "application service identity mismatch"
    );
    {
        let owner = signer
            .lock()
            .map_err(|_| anyhow::anyhow!("signer actor lock poisoned"))?;
        ensure!(
            !signer_status(&owner).fenced
                && Arc::ptr_eq(&owner.service, &service)
                && owner.config.genesis_hash == config.genesis.target.identity.genesis.0.0,
            "application signer/service binding mismatch"
        );
    }
    let cursor = opaque_record_cursor(&replay_repository)?;
    let mut application = ConsensusApplication {
        acceptance_poison_used: std::sync::atomic::AtomicBool::new(false),
        config,
        repository,
        service,
        replay_repository,
        replay_cursor: cursor,
        signer,
        completed: None,
        completed_cursor: None,
        retained_decision: None,
        pending: None,
        replayed_height: None,
        approvals: create_approval_registry(),
        fenced: false,
        #[cfg(test)]
        simulated_failure: None,
    };
    if let Err(error) =
        crate::consensus::application::replay::reconcile_application_replay(&mut application)
    {
        crate::consensus::application::safety::fence_application(&mut application);
        return Err(error);
    }
    Ok(application)
}
