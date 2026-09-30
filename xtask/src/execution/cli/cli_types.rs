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
