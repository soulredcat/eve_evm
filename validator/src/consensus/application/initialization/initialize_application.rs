// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::{
    application::{ConsensusApplication, context::native_application_hash},
    transport::peer::{AuthenticatedEnginePeer, ensure_application_engine_peer},
};
use anyhow::{Result, ensure};
use eve_consensus_comet::wire::tendermint::abci::{RequestInitChain, ResponseInitChain};
use eve_storage::state::read_state_service;

pub(in crate::consensus) fn initialize_application(
    application: &mut ConsensusApplication,
    peer: &AuthenticatedEnginePeer,
    request: &RequestInitChain,
) -> Result<ResponseInitChain> {
    ensure_application_engine_peer(peer)?;
    ensure!(!application.fenced, "application fenced");
    let current = read_state_service(&application.service)?;
    ensure!(
        current.commit().target == application.config.genesis.target
            && application.completed.is_none()
            && application.retained_decision.is_none(),
        "InitChain after local application advancement"
    );
    ensure!(
        request.initial_height == 1
            && request.chain_id == application.config.genesis.target.identity.network_name
            && request.time.as_ref() == Some(&application.config.initial_time)
            && request.consensus_params.as_ref() == Some(&application.config.consensus_params),
        "native InitChain identity/height/parameters mismatch"
    );
    ensure!(
        request.validators == application.config.initial_validators,
        "native InitChain validators differ from canonical root configuration"
    );
    ensure!(
        request.app_state_bytes.len() <= 1_048_576,
        "native InitChain app state byte limit"
    );
    let state: serde_json::Value = serde_json::from_slice(&request.app_state_bytes)?;
    ensure!(
        state == application.config.expected_app_state,
        "native InitChain public specification mismatch"
    );
    Ok(ResponseInitChain {
        consensus_params: Some(application.config.consensus_params.clone()),
        validators: application.config.initial_validators.clone(),
        app_hash: native_application_hash(&application.config.genesis.target)?,
    })
}
