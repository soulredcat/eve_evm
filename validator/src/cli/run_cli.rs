// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::types::{ValidatorCli, ValidatorCommand};
use crate::{init_development_validator, run_development_validator};
use anyhow::Result;
use clap::Parser;

pub fn run_cli() -> Result<()> {
    match ValidatorCli::parse().command {
        ValidatorCommand::InitDev(config) => {
            let initialized = init_development_validator(&config)?;
            println!("{}", serde_json::to_string(&initialized)?);
            Ok(())
        }
        ValidatorCommand::ServeDev(config) => run_development_validator(config),
    }
}
