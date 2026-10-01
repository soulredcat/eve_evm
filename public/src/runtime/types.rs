// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use clap::{Args, Parser, Subcommand};
use std::{net::SocketAddr, path::PathBuf};
#[derive(Clone, Debug, Args)]
pub struct DevelopmentPublicConfig {
    #[arg(long)]
    pub root: PathBuf,
    #[arg(long)]
    pub data: PathBuf,
    #[arg(long)]
    pub genesis: PathBuf,
    #[arg(long)]
    pub mode: String,
    #[arg(long)]
    pub acknowledge_unsafe_development: bool,
    #[arg(long, default_value = "127.0.0.1:8545")]
    pub http_address: SocketAddr,
    #[arg(long, default_value = "127.0.0.1:8546")]
    pub ws_address: SocketAddr,
    #[arg(long, default_value_t = 1000)]
    pub block_interval_ms: u64,
    #[arg(long)]
    pub allow_external_bind: bool,
    #[arg(long, default_value_t = 1)]
    pub zone_id: u16,
}
#[derive(Parser)]
pub(crate) struct PublicCli {
    #[command(subcommand)]
    pub command: PublicCommand,
}
#[derive(Subcommand)]
pub(crate) enum PublicCommand {
    ServeDev(DevelopmentPublicConfig),
}
