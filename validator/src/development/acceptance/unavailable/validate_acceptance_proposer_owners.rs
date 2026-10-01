// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
use alloy_primitives::Address;
use anyhow::{Result, ensure};
use eve_consensus_comet::consensus::certificates::ClassicalValidator;
use std::collections::BTreeMap;

pub(crate) fn validate_acceptance_proposer_owners(
    _fixture: Option<&AcceptanceFixture>,
    initial: &[ClassicalValidator],
    owners: &BTreeMap<[u8; 20], Address>,
) -> Result<()> {
    ensure!(
        owners.len() == initial.len(),
        "unexpected proposer-owner mappings"
    );
    Ok(())
}
