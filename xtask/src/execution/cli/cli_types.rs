use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(about = "EVE repository verification tools")]
pub struct Arguments {
    #[command(subcommand)]
    pub command: TaskCommand,
}

#[derive(Subcommand)]
pub enum TaskCommand {
    /// Provision digest-pinned Linux reference tools in ignored local storage.
    ProvisionTools {
        #[arg(long, default_value = ".")]
        root: PathBuf,
        #[arg(long, default_value = "config/tool-pins.toml")]
        pins: PathBuf,
        #[arg(long, default_value = "local-tests/toolchain-b0")]
        output: PathBuf,
        #[arg(long, default_value_t = 2)]
        jobs: usize,
    },
    /// Run a registered gate; unsupported or missing coverage fails.
    Verify {
        #[arg(long, default_value = ".")]
        root: PathBuf,
        #[arg(long, conflicts_with_all = ["security", "interop", "all"])]
        bulk: Option<String>,
        #[arg(long, conflicts_with_all = ["bulk", "interop", "all"])]
        security: Option<String>,
        #[arg(long, conflicts_with_all = ["bulk", "security", "all"])]
        interop: Option<String>,
        #[arg(long)]
        all: bool,
    },
    /// Check all tracked and new first-party files against plan 25.
    CheckStructure {
        #[arg(long, default_value = ".")]
        root: PathBuf,
        #[arg(long, default_value = "config/structure-policy.toml")]
        policy: PathBuf,
        #[arg(long)]
        report: Option<PathBuf>,
    },
}
