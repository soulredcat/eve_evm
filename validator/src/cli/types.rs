// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use crate::DevelopmentValidatorConfig;
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub(super) struct ValidatorCli {
    #[command(subcommand)]
    pub command: ValidatorCommand,
}

#[derive(Subcommand)]
pub(super) enum ValidatorCommand {
    InitDev(DevelopmentValidatorConfig),
    ServeDev(DevelopmentValidatorConfig),
}
