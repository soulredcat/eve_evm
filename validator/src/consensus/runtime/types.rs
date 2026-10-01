// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::consensus::transport::peer::EnginePeerShutdown;
use crate::{
    consensus::{
        application::ConsensusApplication, approval::ApprovalRegistry, signing::DurableSigner,
    },
    development::{config::DevelopmentValidatorConfig, engine::OwnedEngine},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    net::SocketAddr,
    sync::{Arc, Mutex, atomic::AtomicBool},
};

#[derive(Clone, Debug, Serialize)]
pub struct DevelopmentValidatorInitialization {
    pub node_id: String,
    pub chain_id: String,
    pub genesis_hash: String,
    pub rpc_address: SocketAddr,
    pub p2p_address: SocketAddr,
    pub zone_id: u16,
    pub security_profile: &'static str,
}

#[derive(Deserialize, Serialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(in crate::consensus::runtime) struct NodeIdentity {
    pub schema: u8,
    pub genesis_hash: String,
    pub public_key: String,
    pub engine_sha256: String,
}

pub(in crate::consensus::runtime) struct NodeAssembly {
    pub config: DevelopmentValidatorConfig,
    pub initialization: DevelopmentValidatorInitialization,
    pub digest: [u8; 32],
    pub application: Arc<Mutex<ConsensusApplication>>,
    pub signer: Arc<Mutex<DurableSigner>>,
    pub registry: Arc<ApprovalRegistry>,
    pub lease: File,
}

#[derive(Default)]
pub(in crate::consensus::runtime) struct HandshakeProgress {
    pub info: Option<(i64, Vec<u8>)>,
    pub initialized: bool,
    pub public_key: bool,
    pub ping: bool,
    pub signing_activity: bool,
}

pub(in crate::consensus::runtime) struct RunningNode {
    pub assembly: NodeAssembly,
    pub engine: Mutex<OwnedEngine>,
    pub stop: AtomicBool,
    pub handshake: Mutex<HandshakeProgress>,
    pub failure: Mutex<Option<anyhow::Error>>,
    pub shutdown: Mutex<ChannelShutdown>,
    pub signer_requested: Mutex<Option<super::diagnostics::SignerPosition>>,
}

#[derive(Default)]
pub(in crate::consensus::runtime) struct ChannelShutdown {
    pub application: Vec<EnginePeerShutdown>,
    pub signer: Option<EnginePeerShutdown>,
}

pub(in crate::consensus::runtime) struct ChannelWorkerGuard<'a> {
    pub node: &'a RunningNode,
}

#[derive(Serialize)]
pub(in crate::consensus::runtime) struct NodeReadiness<'a> {
    pub event: &'static str,
    pub initialization: &'a DevelopmentValidatorInitialization,
    pub application_height: i64,
    pub application_hash: String,
    pub consensus_finality: bool,
    pub signer_last_height: Option<i64>,
    pub signer_last_round: Option<i32>,
    pub signer_last_step: Option<u8>,
    pub execution_approvals: ApprovalReadiness,
}

#[derive(Serialize)]
pub(in crate::consensus::runtime) struct ApprovalReadiness {
    pub prepared_states: usize,
    pub retained_proposals: usize,
    pub retained_proposal_bytes: usize,
    pub current_candidate: Option<String>,
    pub protected_prevote: Option<String>,
    pub protected_precommit: Option<String>,
}
