// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use clap::{Args, Parser, Subcommand};
use std::{net::SocketAddr, path::PathBuf};

#[derive(Parser)]
#[command(about = "Local development composition; master storage never grants validator finality")]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Args)]
pub struct DevelopmentOptions {
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
}

#[derive(Subcommand)]
pub enum Command {
    /// Independently verify and durably archive validator-finalized classical-development history.
    FollowDev {
        #[command(flatten)]
        options: crate::sync::MasterFollowerOptions,
    },
    /// Compose bounded public RPC and a local producer without validator finality.
    ServeDev {
        #[command(flatten)]
        options: DevelopmentOptions,
        #[command(flatten)]
        listeners: ListenerOptions,
    },
    /// Export one complete locally durable view into a new ignored snapshot directory.
    SnapshotDev {
        #[command(flatten)]
        options: DevelopmentOptions,
        #[arg(long)]
        output: PathBuf,
    },
    /// Validate a local snapshot and activate it in a new development namespace.
    RestoreDev {
        #[command(flatten)]
        options: DevelopmentOptions,
        #[arg(long)]
        source: PathBuf,
    },
    /// Initialize bounded fake-asset development state in a new ignored namespace.
    InitDev {
        #[command(flatten)]
        options: DevelopmentOptions,
    },
    /// Reconcile and display local durable state without claiming authenticated finality.
    InspectDev {
        #[command(flatten)]
        options: DevelopmentOptions,
    },
    /// Execute real signed EVM envelopes with a local test producer and atomically persist them.
    ApplyDev {
        #[command(flatten)]
        options: DevelopmentOptions,
        #[arg(long)]
        block: PathBuf,
    },
}

#[derive(Args)]
pub struct ListenerOptions {
    #[arg(long, default_value = "127.0.0.1:8545")]
    pub http_address: SocketAddr,
    #[arg(long, default_value = "127.0.0.1:8546")]
    pub ws_address: SocketAddr,
    #[arg(long, default_value_t = 1_000)]
    pub block_interval_ms: u64,
    #[arg(long)]
    pub allow_external_bind: bool,
    #[arg(long, default_value_t = 1)]
    pub zone_id: u16,
}
