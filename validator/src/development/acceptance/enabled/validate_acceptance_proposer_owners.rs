// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
use alloy_primitives::Address;
use anyhow::{Result, ensure};
use eve_consensus_comet::consensus::certificates::{ClassicalValidator, validator_address};
use std::collections::BTreeMap;

/// Initial mappings are checked against canonical genesis; only sealed future mappings may extend them.
pub(crate) fn validate_acceptance_proposer_owners(
    fixture: Option<&AcceptanceFixture>,
    initial: &[ClassicalValidator],
    owners: &BTreeMap<[u8; 20], Address>,
) -> Result<()> {
    let mut expected = initial.len();
    if let Some(fixture) = fixture {
        expected += fixture.future.len();
        for (key, owner) in &fixture.future {
            ensure!(
                owners.get(&validator_address(key)) == Some(owner),
                "missing or changed acceptance future proposer owner"
            );
        }
    }
    ensure!(
        owners.len() == expected,
        "unexpected proposer-owner mappings outside bound enrollment"
    );
    Ok(())
}
