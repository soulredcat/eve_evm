use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

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
