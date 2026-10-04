// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use clap::Args;
use eve_state::DevelopmentGenesis;
use std::{net::SocketAddr, path::PathBuf};
use tokio::sync::OwnedSemaphorePermit;

#[derive(Args)]
pub struct MasterFollowerOptions {
    #[arg(long, default_value = ".")]
    pub root: PathBuf,
    #[arg(long)]
    pub data: PathBuf,
    #[arg(long)]
    pub genesis: PathBuf,
    #[arg(long)]
    pub mode: String,
    #[arg(long)]
    pub acknowledge_unsafe_development: bool,
    #[arg(long)]
    pub native_address: SocketAddr,
    #[arg(long)]
    pub through_height: u64,
    #[arg(long, default_value_t = 5_000)]
    pub timeout_ms: u64,
    #[arg(long, default_value_t = 262_144)]
    pub maximum_response_bytes: usize,
    #[arg(long, default_value_t = 262_144)]
    pub maximum_chunk_bytes: u32,
}

pub(super) struct ChargedMasterGenesis {
    pub genesis: DevelopmentGenesis,
    pub _lease: OwnedSemaphorePermit,
}
