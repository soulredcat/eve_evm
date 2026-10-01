// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::{
    run_development_public,
    types::{PublicCli, PublicCommand},
};
use anyhow::Result;
use clap::Parser;
pub async fn run_public_runtime() -> Result<()> {
    match PublicCli::parse().command {
        PublicCommand::ServeDev(config) => run_development_public(config).await,
    }
}
