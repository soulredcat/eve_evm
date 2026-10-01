// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::super::proxy::ProxySet;
use alloy_primitives::Address;
use eve_state::StateCommit;
use k256::ecdsa::SigningKey;
use std::{
    net::{SocketAddr, TcpListener},
    path::PathBuf,
    process::Child,
};

#[derive(Default)]
pub(crate) struct ClusterOptions {
    pub transitions: bool,
    pub poison: bool,
    pub observer: bool,
}

pub(crate) struct Node {
    pub data: PathBuf,
    pub seed: PathBuf,
    pub rpc: SocketAddr,
    pub p2p: SocketAddr,
    pub advertised: SocketAddr,
    pub node_id: String,
    pub public_key: [u8; 32],
    pub owner: Address,
    pub child: Option<Child>,
    pub engine_pid: Option<u32>,
    pub engine_start: Option<u64>,
}

pub(crate) struct Cluster {
    pub namespace: Option<tempfile::TempDir>,
    pub retain_failed_namespace: bool,
    pub binary_sha256: String,
    pub artifact: PathBuf,
    pub binary: PathBuf,
    pub comet: PathBuf,
    pub comet_sha: String,
    pub genesis_path: PathBuf,
    pub genesis: StateCommit,
    pub chain_id: String,
    pub authority: SigningKey,
    pub authority_address: Address,
    pub fixture_path: Option<PathBuf>,
    pub fixture_digest: [u8; 32],
    pub nodes: Vec<Node>,
    pub proxies: ProxySet,
    pub reservations: Vec<TcpListener>,
}
