// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use anyhow::{Result, ensure};
use eve_protocol_config::genesis::DevelopmentGenesis;

/// Never run the same acceptance genesis under two different application rules.
pub(super) fn validate_acceptance_profile(
    genesis: &DevelopmentGenesis,
    supplied: bool,
) -> Result<()> {
    eve_protocol_config::genesis::validate_development_genesis(
        eve_protocol_config::network::LaunchMode::Development,
        genesis,
    )
    .map_err(|_| {
        anyhow::anyhow!("acceptance fixture requires a valid classical development genesis")
    })?;
    let tag = b"EVE_B3_ACCEPTANCE_V1";
    let marker = genesis.accounts.iter().any(|account| {
        let code = account.code.as_ref();
        code.len() >= 20 + tag.len() + 32
            && &code[code.len() - 32 - tag.len()..code.len() - 32] == tag
    });
    ensure!(
        marker == supplied && (!marker || cfg!(feature = "development-acceptance")),
        "acceptance genesis requires its exact manifest and explicit disposable build feature"
    );
    Ok(())
}
