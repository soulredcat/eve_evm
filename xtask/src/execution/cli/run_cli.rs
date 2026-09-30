use super::cli_types::{Arguments, TaskCommand};
use crate::structure::checking::check_structure::check_structure;
use anyhow::{Result, bail};
use clap::Parser;

pub fn run_cli() -> Result<()> {
    let arguments = Arguments::parse();
    match arguments.command {
        TaskCommand::CheckStructure {
            root,
            policy,
            report,
        } => {
            let result = check_structure(&root, &policy)?;
            let encoded = serde_json::to_string_pretty(&result)?;
            if let Some(path) = report {
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(path, &encoded)?;
            }
            println!("{encoded}");
            if !result.violations.is_empty() {
                bail!(
                    "Structure gate failed with {} violation(s)",
                    result.violations.len()
                );
            }
        }
    }
    Ok(())
}
