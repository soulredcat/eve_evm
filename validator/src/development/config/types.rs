// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use clap::Args;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Clone, Args)]
pub struct DevelopmentValidatorConfig {
    #[arg(long)]
    pub genesis: PathBuf,
    #[arg(long)]
    pub data: PathBuf,
    #[arg(long)]
    pub signing_seed: PathBuf,
    #[arg(long)]
    pub comet_binary: PathBuf,
    #[arg(long)]
    pub comet_sha256: String,
    #[arg(long)]
    pub acknowledge_unsafe_development: bool,
    #[arg(long, default_value = "127.0.0.1:26657")]
    pub rpc_address: SocketAddr,
    #[arg(long, default_value = "127.0.0.1:26656")]
    pub p2p_address: SocketAddr,
    #[arg(long)]
    pub advertised_p2p_address: Option<SocketAddr>,
    #[arg(long)]
    pub acceptance_fixture: Option<PathBuf>,
    #[arg(long, default_value = "")]
    pub persistent_peers: String,
    #[arg(long, default_value_t = 1)]
    pub zone_id: u16,
    /// Independent CLASSICAL_DEV retained-query working estimate, not proposal RAM.
    #[arg(long, default_value_t = 64 * 1_048_576)]
    pub recovery_query_working_bytes: usize,
}
