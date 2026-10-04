// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use clap::Args;
use std::{net::SocketAddr, path::PathBuf};

#[derive(Clone, Debug, Args)]
pub struct DevelopmentFollowerConfig {
    #[arg(long)]
    pub root: PathBuf,
    #[arg(long)]
    pub data: PathBuf,
    #[arg(long)]
    pub genesis: PathBuf,
    #[arg(long)]
    pub acknowledge_unsafe_development: bool,
    #[arg(long)]
    pub validator_address: SocketAddr,
    #[arg(long, default_value = "127.0.0.1:8545")]
    pub http_address: SocketAddr,
    #[arg(long, default_value = "127.0.0.1:8546")]
    pub ws_address: SocketAddr,
    #[arg(long, default_value = "public-1")]
    pub node_name: String,
    #[arg(long, default_value_t = 1)]
    pub zone_id: u16,
    #[arg(long, default_value_t = 100)]
    pub poll_interval_ms: u64,
    #[arg(long)]
    pub checkpoint_height: Option<u64>,
}
