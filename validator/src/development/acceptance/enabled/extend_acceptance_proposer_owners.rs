// SPDX-FileCopyrightText: 2026 Redcat
// SPDX-License-Identifier: LicenseRef-Redcat-Permission-Only
// Use requires prior written permission from Redcat.

use super::AcceptanceFixture;
use alloy_primitives::Address;
use anyhow::{Result, ensure};
use eve_consensus_comet::consensus::certificates::validator_address;
use std::collections::BTreeMap;

pub(crate) fn extend_acceptance_proposer_owners(
    fixture: Option<&AcceptanceFixture>,
    owners: &mut BTreeMap<[u8; 20], Address>,
) -> Result<()> {
    if let Some(fixture) = fixture {
        for (key, owner) in &fixture.future {
            ensure!(
                owners.insert(validator_address(key), *owner).is_none(),
                "acceptance/native proposer identity collision"
            );
        }
    }
    Ok(())
}
